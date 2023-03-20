# Copyright © Aptos Foundation
# SPDX-License-Identifier: Apache-2.0

import asyncio
import os
import time

from aptos_sdk.account import Account
from aptos_sdk.async_client import ClientConfig, FaucetClient, RestClient
from aptos_sdk.transactions import Script, ScriptArgument, TransactionPayload

from .common import FAUCET_URL, NODE_URL


async def main():
    client_config = ClientConfig()
    # Toggle to benchmark
    client_config.http2 = False 
    client_config.http2 = True
    rest_client = RestClient(NODE_URL, client_config)
    faucet_client = FaucetClient(FAUCET_URL, rest_client)

    start = time.time()
    print(f"Starting...")

    num_accounts = 50
    accounts = []
    for _ in range(num_accounts):
        accounts.append(Account.generate())

    print(f"Accounts generated at {time.time() - start}")

    funds = []
    for account in accounts:
        funds.append(faucet_client.fund_account(account.address(), 100_000_000))
    await asyncio.gather(*funds)

    print(f"Funded accounts at {time.time() - start}")

    balances = []
    for _ in range(100):
        for account in accounts:
            balances.append(rest_client.account_balance(account.address()))
    await asyncio.gather(*balances)

    print(f"Accounts checked at {time.time() - start}")

    upper = num_accounts // 2
    txn_hashes = []
    for idx in range(upper):
        sender = accounts[idx]
        receiver = accounts[idx+upper].address()
        txn_hashes.append(rest_client.bcs_transfer(sender, receiver, 1))
    txn_hashes = await asyncio.gather(*txn_hashes)

    print(f"Transactions submitted at {time.time() - start}")

    wait_for = []
    for txn_hash in txn_hashes:
        wait_for.append(rest_client.wait_for_transaction(txn_hash))
    await asyncio.gather(*wait_for)

    print(f"Transactions committed at {time.time() - start}")

    await rest_client.close()


if __name__ == "__main__":
    asyncio.run(main())
