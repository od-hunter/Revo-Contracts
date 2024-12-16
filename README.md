
## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
    - [Installing Project](#installing-Project)
    - [Configuring Smart Contracts](#configuring-smart-contracts)
3. [Wallet Configuration](#wallet-configuration)
4. [Compilation and Deployment](#compilation-and-deployment)
5. [Testing and Execution](#testing-and-execution)
6. [Troubleshooting](#troubleshooting)

---

## Prerequisites
   Before getting started, make sure you have the following installed on your system:

### 1. Installing Rust

- ### Linux, macOS, or Unix-like Systems
  If you're using macOS, Linux, or any other Unix-like system, the simplest method to install Rust is by using `rustup`. Install it with the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- ### Windows
  On Windows, download and run `rustup-init.exe`. You can proceed with the default setup by pressing `Enter`.
  You can also follow the official Rust guide [here](https://www.rust-lang.org/tools/install).

- ### Install the wasm32 target.
       After installing Rust, add the `wasm32-unknown-unknown` target:
```bash
rustup target add wasm32-unknown-unknown
```

## 2. Install Stellar CLI
- There are a few ways to install the [latest released version](https://github.com/stellar/stellar-cli/releases) of Stellar CLI.
- The toolset installed with Rust allows you to use the `cargo` command in the terminal to install the Stellar CLI.

- ### Install with cargo from source:
```sh
cargo install --locked stellar-cli --features opt
```

- ### Install with Homebrew (macOS, Linux):
```sh
brew install stellar-cli
```

## Environment Setup

### 1. Installing Project
-  Clone the repository:
   ```bash
   git clone https://github.com/<username>/Revo-Contracts.git
   cd ./Revo-Contracts
   ```

### Configuring Smart Contracts

1. To build a smart contract to deploy or run: 
   ```bash
   stellar contract build
   ```
2. Run the Tests:
   ```bash
   cargo test
   ```
---

## Wallet Configuration
1. Install the Stellar Wallet (e.g., [Freighter Wallet](https://www.freighter.app/)).
2. Create a wallet and save the secret keys securely.
3. Connect the wallet to the Stellar test network.
   
## Compilation and Deployment

### 1. Build contract
Once you have fully set up the contract in your local environment, installed all the necessary tools, and properly configured your user for the testnet, you will be ready to perform the initial deployment to the Testnet and run tests directly on the contract.

The first step is to compile the contract and generate the `.wasm` file, which can be done using the following command:

```bash
stellar contract build
```

### 2. Install contract

Before deploying the contract, you must first install it. This means uploading a version of your code to the Stellar network, which you can later use for deployment.

When you execute the following command with the parameters specific to your local environment, it will return a hash. You will need to save this hash, as it will be required in the next step.

- ### macOS/Linux
```bash
stellar contract install \
   --network <network> \
   --source <source_account> \
   --wasm <path_to_wasm_file>
```

- ### Windows (PowerShell)
```bash
stellar contract install `
   --network <network> `
   --source <source_account> `
   --wasm <path_to_wasm_file>
```

Where:

- `<network>` is the name of the network you are working on (e.g., testnet).
- `<source_account>` is the account from which the installation will be made (you need to provide your own account).
- `<path_to_wasm_file>` is the path to the `.wasm` file generated when compiling the contract."

Response:

```
d36cd70c3b9c999e172ecc4648e616d9a49fd5dbbae8c28bef0b90bbb32fc762
```

### 3.Deploy contract

- Finally, to deploy the contract, you will need to use the output from the previous command as the input parameter for this command.

- Once you execute this command, you will receive another hash, which will be the contract ID. With this ID, you can query platforms such as https://stellar.expert/explorer/testnet and continuously monitor the interactions made with the deployed contract.

- ### macOS/Linux
```bash
stellar contract deploy \
   --wasm-hash <wasm_hash> \
   --source <source_account> \
   --network <network>
```

- ### Windows (PowerShell)
```bash
stellar contract deploy `
   --wasm-hash <wasm_hash> `
   --source <source_account> `
   --network <network>
```

Where:

- `<wasm_hash>` is the hash of the `.wasm` file generated during the contract installation.
- `<source_account>` is the account from which the deployment will be made.
- `<network>` is the network you are working on (e.g., testnet).
---

## Testing and Execution

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

## Troubleshooting
### Common Issues and Fixes
1. ##Rust Installation Issues##
   - Ensure `cargo` is in your system PATH.

2. ##Soroban CLI Errors##
   - Verify the version compatibility of Soroban CLI.
   - Use the `--help` flag to get details of commands:
     ```bash
     soroban --help
     ```

3. ##Wallet Connectivity##
   - Double-check network configuration (testnet/mainnet).

---