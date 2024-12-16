# Stellar Smart Contracts - Beginner-Friendly Guide

Welcome to the Stellar Smart Contracts setup guide! This document will help you get started with deploying Smart Contracts using Stellar. Follow the steps below to configure your environment, deploy contracts, and test them seamlessly.

---

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
    - [Installing Stellar Tools](#installing-stellar-tools)
    - [Configuring Smart Contracts](#configuring-smart-contracts)
3. [Wallet Configuration](#wallet-configuration)
4. [Compilation and Deployment](#compilation-and-deployment)
5. [Testing and Execution](#testing-and-execution)
6. [Troubleshooting](#troubleshooting)
7. [Practical Example](#practical-example)

---

## Prerequisites
Before getting started, make sure you have the following installed on your system:

1. **Node.js and npm**
   - Install Node.js from [Node.js Official Site](https://nodejs.org/).
   - Verify installation:
     ```bash
     node -v
     npm -v
     ```
2. **Git**
   - Download and install Git from [Git Official Site](https://git-scm.com/).
   - Verify installation:
     ```bash
     git --version
     ```
3. **Stellar SDK**
   - Install Stellar SDK using npm:
     ```bash
     npm install stellar-sdk
     ```
4. **Rust Toolchain**
   - Install Rust using rustup:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Install the wasm32 target:
     ```bash
     rustup target add wasm32-unknown-unknown
     ```
5. **Stellar CLI**
   - Install Stellar CLI using Homebrew (macOS/Linux):
     ```bash
     brew install stellar-cli
     ```
   - Or install with Cargo:
     ```bash
     cargo install --locked stellar-cli@22.0.1 --features opt
     ```

---

## Environment Setup

### Installing Stellar Tools
1. Clone the repository:
   ```bash
   git clone https://github.com/Crypto-Jaguars/Revo-Contracts.git
   cd ./Revo-Contracts
   ```
2. Install dependencies:
   ```bash
   npm install
   ```

### Configuring Smart Contracts
Stellar smart contracts are written using [Soroban](https://soroban.stellar.org/).

1. Install the Soroban CLI:
   ```bash
   cargo install --locked --version <version-number> soroban-cli
   ```
   > Note: Ensure you have Rust installed. You can install Rust from [Rust Official Site](https://www.rust-lang.org/).

2. Verify Soroban installation:
   ```bash
   soroban --version
   ```
3. Initialize Soroban:
   ```bash
   soroban config init
   ```
4. Configure Stellar CLI for testnet:
   ```bash
   stellar keys generate --global alice --network testnet --fund
   ```
   Verify the identity with:
   ```bash
   stellar keys address alice
   ```

---

## Wallet Configuration
1. Install the Stellar Wallet (e.g., [Freighter Wallet](https://www.freighter.app/)).
2. Create a wallet and save the secret keys securely.
3. Connect the wallet to the Stellar test network.
   - Update the Soroban configuration to use the testnet:
     ```bash
     soroban config set network-url https://horizon-testnet.stellar.org
     ```

---

## Compilation and Deployment
1. Compile the smart contract:
   ```bash
   soroban contract build
   ```
2. Deploy the contract:
   ```bash
   soroban contract deploy --wasm-file <path-to-compiled-wasm>
   ```
3. Save the contract ID for future use.

---

## Testing and Execution
1. Interact with the deployed contract:
   ```bash
   soroban contract invoke --id <contract-id> --fn <function-name> --arg <arguments>
   ```
2. Example:
   ```bash
   soroban contract invoke --id 123456 --fn say_hello --arg "World"
   ```

---

## Troubleshooting
### Common Issues and Fixes
1. **Rust Installation Issues**
   - Ensure `cargo` is in your system PATH.

2. **Soroban CLI Errors**
   - Verify the version compatibility of Soroban CLI.
   - Use the `--help` flag to get details of commands:
     ```bash
     soroban --help
     ```

3. **Wallet Connectivity**
   - Double-check network configuration (testnet/mainnet).

---

## Practical Example

Let’s walk through deploying a simple smart contract.

1. Write a basic Soroban contract (e.g., `hello_world.rs`):
   ```rust
   #[macro_use]
   extern crate soroban_sdk;

   #[contract]
   pub struct HelloWorld;

   #[contractimpl]
   impl HelloWorld {
       pub fn say_hello(to: String) -> String {
           format!("Hello, {}!", to)
       }
   }
   ```

2. Compile and deploy:
   ```bash
   soroban contract build
   soroban contract deploy --wasm-file target/wasm32-unknown-unknown/release/hello_world.wasm
   ```
3. Invoke the contract:
   ```bash
   soroban contract invoke --id <contract-id> --fn say_hello --arg "Aayush"
   ```
   Output:
   ```
   "Hello, Aayush!"
   ```

---

## Conclusion
You are now equipped to set up, deploy, and test smart contracts on Stellar. If you face any issues, refer to the troubleshooting section or the [Stellar Official Guide](https://developers.stellar.org/).
