// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context as AnyhowContext, Result};
use aptos_api::context::Context;
use aptos_protos::api::v2::{
    api_v2_server::{ApiV2, ApiV2Server},
    GetAccountModuleRequest, GetAccountModuleResponse,
    FILE_DESCRIPTOR_SET,
};
use poem::{endpoint::TowerCompatExt, IntoEndpoint, Route};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use sync_wrapper::SyncWrapper;
use aptos_logger::info;

// todo
#[derive(Clone)]
pub struct ApiV2Service {
    pub context: Arc<Context>,
}

// TODO: Temporary until issues in build_api_v2_service below are solved.
pub fn build_api_v2_runtime(context: Arc<Context>, runtime: &tokio::runtime::Runtime) -> Result<()> {
    let service = ApiV2Service { context };

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .context("Failed to build reflection service")?;

    runtime.spawn(async move {
        let address = "0.0.0.0:50052".parse().unwrap();
        Server::builder()
            .add_service(ApiV2Server::new(service))
            .add_service(reflection_service)
            .serve(address)
            .await.expect("Failed to start API v2 server");
        info!(address = address, "[indexer-grpc] Started GRPC server");
    });

    Ok(())
}

pub fn build_api_v2_service(context: Arc<Context>) -> Result<impl IntoEndpoint> {
    /*
    let service = ApiV2Service { context };

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .context("Failed to build reflection service")?;

    let tower_service = Server::builder()
        .add_service(ApiV2Server::new(service))
        .into_service();

    // https://github.com/tower-rs/tower/issues/691
    let tower_service = SyncWrapper::new(tower::util::BoxCloneService::new(tower_service));

    let tower_service = tower_service.compat();

    // https://github.com/poem-web/poem/issues/536
    // https://github.com/hyperium/tonic/issues/1322
    let routes = Route::new().nest("/", tower_service);
    */

    // TODO: Temporary
    let routes = Route::new();

    Ok(routes)
}

#[tonic::async_trait]
impl ApiV2 for ApiV2Service {
    async fn get_account_module(
        &self,
        request: Request<GetAccountModuleRequest>,
    ) -> Result<Response<GetAccountModuleResponse>, Status> {
        unimplemented!();
    }
}
