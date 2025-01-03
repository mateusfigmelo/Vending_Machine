# Vending Machine Smart Contract

This repository contains the implementation of a Vending Machine smart contract written in Rust, designed to run on the CosmWasm platform.

## Features

- Manages stock for three types of items: chocolate, water, and chips.
- Allows users to purchase items if they are in stock.
- Enables the contract owner to refill the vending machine.
- Provides querying capabilities to check the current stock of items.

---

## Table of Contents

1. [Setup](#setup)
2. [Build and Test](#build-and-test)
3. [Deploy and Interact with the Contract](#deploy-and-interact-with-the-contract)
4. [Code Structure](#code-structure)
5. [License](#license)

---

## Setup

### Prerequisites

Ensure you have the following tools installed:

- [Rust](https://www.rust-lang.org/) installation

    To work with CosmWasm smart contract, you will need Rust installed on your machine. 
    If you don't have it, you can find installation instructions on the Rust website.
    I assume you are working with the stable Rust channel(It is recommended to use rust stable version less than 1.82.0 for compability with Cosmwasm Check) in this repo.
    Additionally, you will need the Wasm rust compiler backend installed to build Wasm binaries. To install it, run:
    
    ```bash
    rustup target add wasm32-unknown-unknown
    ```

- [The cosmwasm-check utility](https://github.com/CosmWasm/cosmwasm/tree/main/packages/check)

    An additional helpful tool for building smart contracts is the cosmwasm-check utility. It allows you to check if the wasm binary is a proper smart contract ready to upload into the blockchain. You can install it using cargo:
    ```bash
    cargo install cosmwasm-check
    ```
    If the installation succeeds, you should be able to execute the utility from your command line.
    ```bash
    cosmwasm-check --version
    ```

- [wasmd](https://github.com/CosmWasm/wasmd)

    To build and install `wasmd`, Go is required. If you haven't installed Go yet, you can set it up by visiting the [Go download and install page](https://go.dev/doc/install).
    To setup `wasmd`:
    ```bash
    git clone https://github.com/CosmWasm/wasmd.git && cd wasmd
    git checkout v0.52.0
    make install
    ```
    You can verify the version of Wasmd you have once it is installed:
    ```bash
    wasmd version
    ```
---

## Build and Test

### Building the Smart Contract

1. Navigate to the root directory of the project.
2. Build the contract using:

   ```bash
   cargo wasm
   ```
3. The Wasm file will be located in the `target/wasm32-unknown-unknown/release/` directory.
4. Validate the Wasm binary:

   You should be able to find your output binary in the target/wasm32-unknown-unknown/release/ of the root repo directory - not in the contract directory itself! Now you can check if contract validation passes:
   ```bash
   cosmwasm-check ./target/wasm32-unknown-unknown/release/
   ```


    Additionally, to generate schemas you need to run the command:
    ```bash
    cargo run schema
    ```

### Running Unit Tests

Run unit tests for the contract:

```bash
cargo test
```

---

## Deploy and Interact with the Contract

### Run A Node
    ```bash
    make clean && make start
    ```

### Environment Variables

Set the necessary environment variables in the `.env` file:
Make sure `CHAIN_ID`, `WALLET_NAME` is same as `CHAIN_ID` and `ACCOUNT_ALICE` of `Makefile`.

```env
CHAIN_ID="docs-chain-1"
WALLET_NAME="alice"
GAS="auto"
GAS_ADJUSTMENT="1.3"
CONTRACT_PATH="target/wasm32-unknown-unknown/release/vending_machine.wasm"
CONTRACT_ADDRESS="wasm1..."
```

### Deploying the Smart Contract

Use the `scripts/deploy_contract.rs` script to deploy the contract:

```bash
bash scripts/deploy_contract.rs
```

This script will:

1. Upload the contract to the blockchain.
2. Instantiate the contract with initial stock values (e.g., chocolate: 10, water: 20, chips: 30).
3. Print the deployed contract address.

### Interact with the Contract

Before running a script you need to replace the `CONTRACT_ADDRESS` of `.env` file into an actual deployed address.

Use the `scripts/interact_contract.rs` script for interactions:

```bash
bash scripts/interact_contract.rs
```

Examples of interactions:

1. Query the current stock:

   ```bash
   wasmd query wasm contract-state smart $CONTRACT_ADDRESS '{"ItemsCount":{}}'
   ```

2. Purchase an item:

   ```bash
   wasmd tx wasm execute $CONTRACT_ADDRESS '{"GetItem":{"item_type":"Chocolate"}}' --from alice --chain-id $CHAIN_ID -y
   ```

3. Refill stock (owner-only):

   ```bash
   wasmd tx wasm execute $CONTRACT_ADDRESS '{"Refill":{"chocolate":5,"water":5,"chips":5}}' --from alice --chain-id $CHAIN_ID -y
   ```

---

## Code Structure

- **src/contract.rs**: Core contract logic for instantiation, execution, and querying.
- **src/msg.rs**: Defines messages for instantiation, execution, and queries.
- **src/state.rs**: State management for the vending machine.
- **src/error.rs**: Custom error definitions.
- **src/lib.rs**: Entry point for the smart contract.
- **Makefile**: Commands for setting up and running the local blockchain node.
- **scripts/**: Deployment and interaction scripts.
- **.env**: Environment configuration file.

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

For further assistance, refer to the [CosmWasm documentation](https://docs.cosmwasm.com/).
