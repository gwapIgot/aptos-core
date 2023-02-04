// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{
    network::{DagSender, NetworkSender},
    round_manager::VerifiedEvent,
};
use aptos_channels::aptos_channel;
use aptos_consensus_types::common::Round;
use aptos_consensus_types::node::{CertifiedNode, CertifiedNodeAck, CertifiedNodeRequest};
use aptos_crypto::HashValue;
use aptos_types::PeerId;
use futures::StreamExt;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::time::Duration;
use tokio::{sync::mpsc::Receiver, time};

#[allow(dead_code)]
pub(crate) enum DagDriverCommand {}

// TODO: add enum status (missing node and missing parents). Marge both maps.
#[allow(dead_code)]
pub struct MissingDagNodeData {
    node_source: PeerId,
    node_round: Round,
    immediate_dependencies: HashSet<HashValue>,
    peers_to_request: HashSet<PeerId>,
    need_to_send_request: bool,
}

#[allow(dead_code)]
impl MissingDagNodeData {
    pub fn new(node_source: PeerId, node_round: Round, need_to_send_request: bool) -> Self {
        Self {
            node_source,
            node_round,
            immediate_dependencies: HashSet::new(),
            peers_to_request: HashSet::new(),
            need_to_send_request,
        }
    }

    pub fn node_source(&self) -> PeerId {
        self.node_source
    }

    pub fn node_round(&self) -> Round {
        self.node_round
    }

    pub fn need_to_send_request(&self) -> bool {
        self.need_to_send_request
    }

    pub fn add_peer(&mut self, peer_id: PeerId) {
        self.peers_to_request.insert(peer_id);
    }

    pub fn add_dependency(&mut self, dependency: HashValue) {
        self.immediate_dependencies.insert(dependency);
    }

    pub fn peers_to_request(&self) -> Vec<PeerId> {
        self.peers_to_request.iter().cloned().collect()
    }

    pub fn disable_requests(&mut self) {
        self.need_to_send_request = false;
    }

    pub fn take_dependencies(self) -> HashSet<HashValue> {
        self.immediate_dependencies
    }
}

// TODO: Create new a node once round is ready and pass to RB and push the round to Bullshark. Pull/get proofs from QS.
// TODO: weak links and GC.
// TODO: Timeouts and anchor election! Arc<something> and call it when needed.

#[allow(dead_code)]
pub struct DagDriver {
    my_id: PeerId,
    round: Round,
    network_sender: NetworkSender,
    // TODO: Should we clean more often than once an epoch?
    dag: Vec<HashMap<PeerId, CertifiedNode>>, // TODO: link to yourself and do HashMap<peerid, Vec> to keep the front. Actually might be bad for syncing.
    // TODO: persist both maps
    // the set contains nodes' missing parents
    pending_certified_nodes: HashMap<HashValue, (CertifiedNode, HashSet<HashValue>)>, // TODO: marge this two anf have status in MissingDagNodeData
    missing_certified_nodes: HashMap<HashValue, MissingDagNodeData>, // nodes that are missing in the dag, but might be in pending
}

#[allow(dead_code)]
impl DagDriver {
    fn contains(&self, round: Round, peer_id: PeerId) -> bool {
        self.dag
            .get(round as usize)
            .map(|m| m.contains_key(&peer_id))
            == Some(true)
    }

    fn round_digests(&self, round: Round) -> Option<HashSet<HashValue>> {
        self.dag.get(round as usize).map(|m| {
            m.iter()
                .map(|(_, certified_node)| certified_node.node().digest())
                .collect()
        })
    }

    fn update_pending_nodes(
        &mut self,
        new_dag_node_data: MissingDagNodeData,
        new_dag_mode_digest: HashValue,
    ) {
        for digest in new_dag_node_data.take_dependencies() {
            match self.pending_certified_nodes.entry(digest) {
                Entry::Occupied(mut entry) => {
                    let set = &mut entry.get_mut().1;
                    // let certified_node = &entry.get().0;
                    set.remove(&new_dag_mode_digest);

                    if set.is_empty() {
                        let (certified_node, _) = entry.remove();
                        self.add_to_dag(certified_node);
                    }
                },
                Entry::Vacant(_) => unreachable!("pending node is missing"),
            }
        }
    }

    fn add_to_dag(&mut self, certified_node: CertifiedNode) {
        let round = certified_node.node().round() as usize;
        let digest = certified_node.node().digest();
        assert!(self.dag.len() >= round - 1);
        if self.dag.len() < round {
            self.dag.push(HashMap::new());
        }
        self.dag[round].insert(certified_node.node().source(), certified_node);

        // TODO persist!
        if let Some(missing_dag_node_data) = self.missing_certified_nodes.remove(&digest) {
            self.update_pending_nodes(missing_dag_node_data, digest);
        }

        // TODO: check if round is completed-> start new round and pass current to Bullshark.
    }

    fn add_peers_recursively(&mut self, digest: HashValue, source: PeerId) {
        let missing_parents = match self.pending_certified_nodes.get(&digest) {
            Some((_, set)) => set.clone(),
            None => return,
        };
        for parent_digest in missing_parents {
            match self.missing_certified_nodes.entry(parent_digest) {
                Entry::Occupied(mut entry) => {
                    if entry.get().need_to_send_request() {
                        entry.get_mut().add_peer(source);
                    } else {
                        self.add_peers_recursively(parent_digest, source);
                    }
                },
                Entry::Vacant(_) => unreachable!("node should exist in missing nodes"),
            };
        }
    }

    fn add_to_pending(
        &mut self,
        certified_node: CertifiedNode,
        missing_parents: HashSet<(PeerId, HashValue)>,
    ) {
        let pending_peer_id = certified_node.node().source();
        let pending_digest = certified_node.node().digest();
        let pending_round = certified_node.node().round();
        let missing_parents_digest = missing_parents.iter().map(|(_, digest)| *digest).collect();
        self.pending_certified_nodes
            .insert(pending_digest, (certified_node, missing_parents_digest));
        // TODO: Persist

        for (node_source, digest) in missing_parents {
            let missing_dag_node_data =
                self.missing_certified_nodes
                    .entry(digest)
                    .or_insert(MissingDagNodeData::new(
                        node_source,
                        pending_round - 1,
                        !self.pending_certified_nodes.contains_key(&digest),
                    ));

            missing_dag_node_data.add_dependency(pending_digest);
            missing_dag_node_data.add_peer(pending_peer_id);

            self.add_peers_recursively(digest, pending_peer_id); // Recursively update source_peers.
        }
    }

    async fn remote_fetch_missing_nodes(&self) {
        for (digest, missing_dag_node_data) in self
            .missing_certified_nodes
            .iter()
            .filter(|(_, missing_dag_node_data)| missing_dag_node_data.need_to_send_request())
        {
            let request = CertifiedNodeRequest::new(
                missing_dag_node_data.node_source,
                missing_dag_node_data.node_round,
                *digest,
                self.my_id,
            );
            self.network_sender
                .send_certified_node_request(request, missing_dag_node_data.peers_to_request())
                .await;
        }
    }

    async fn handle_node_request(&mut self, node_request: CertifiedNodeRequest) {
        if self.dag.len() < node_request.node_round() as usize {
            return;
        }

        let certified_node =
            match self.dag[node_request.node_round() as usize].get(&node_request.node_source()) {
                None => return,
                Some(node) => node,
            };

        // TODO: do we need this check? do we need request to have digest?
        if certified_node.node().digest() == node_request.digest() {
            self.network_sender
                .send_certified_node(
                    certified_node.clone(),
                    Some(vec![node_request.requester()]),
                    false,
                )
                .await
        }
    }

    // TODO: call self.dag.try_adding_node(certified_node) -> round ready. Move add to dag/pending into DAG.
    async fn handle_certified_node(&mut self, certified_node: CertifiedNode, ack_required: bool) {

        // TODO: check if dag contains the digest

        // TODO: Move logic to try_add_node in dag.rs. Leave here the contain check, the call, and the ack send. The call should return an option to parents for a new node.
        let prev_round_digest_set = self
            .round_digests(certified_node.node().round() - 1)
            .unwrap_or_default();

        let missing_parents: HashSet<(PeerId, HashValue)> = certified_node
            .node()
            .parents()
            .iter()
            .filter_map(|(peer_id, digest)| {
                (!prev_round_digest_set.contains(digest)).then_some((*peer_id, *digest))
            })
            .collect();

        let digest = certified_node.node().digest();
        let source = certified_node.node().source();
        if missing_parents.is_empty() {
            self.add_to_dag(certified_node); // TODO: should persist inside
            // TODO: call update pending
        } else {
            self.add_to_pending(certified_node, missing_parents); // TODO: should persist inside
        }

        if ack_required {
            let ack = CertifiedNodeAck::new(digest, self.my_id);
            self.network_sender
                .send_certified_node_ack(ack, vec![source])
                .await
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn start(
        &mut self,
        mut network_msg_rx: aptos_channel::Receiver<PeerId, VerifiedEvent>,
        mut command_rx: Receiver<DagDriverCommand>,
    ) {
        let mut interval = time::interval(Duration::from_millis(500)); // time out should be slightly more than one network round trip.
        loop {
            // TODO: shutdown
            tokio::select! {
                biased;

                _ = interval.tick() => {
                self.remote_fetch_missing_nodes().await
            },

            Some(_command) = command_rx.recv() => {
                // TODO: proofs from consensus & other apps.
                // TODO: probably better to pull when time to crete new round.
            },

            Some(msg) = network_msg_rx.next() => {
                    match msg {

                        VerifiedEvent::CertifiedNodeMsg(certified_node, ack_required) => {

                            self.handle_certified_node(*certified_node, ack_required).await;

                        },

                        VerifiedEvent::CertifiedNodeRequestMsg(node_request) => {
                            self.handle_node_request(*node_request).await;
                    }

                    _ => unreachable!("reliable broadcast got wrong messsgae"),
                    }
                },

            }
        }
    }
}
