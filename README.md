
# Maintainers 👨🏻‍🔧:

<table align="center">
  <tr>
    <td align="center">
      <img src="https://avatars.githubusercontent.com/u/176054645?v=4" alt="maintainer 1" width="150" />
      <br /><br />
      <strong>Software Engineer | OSS contributor</strong>
      <br /><br />
      <a href="https://github.com/aguilar1x" target="_blank">Matias</a>
      <br />
      <a href="https://t.me/aguilar1x" target="_blank">Telegram</a>
    </td>    
  </tr>
</table>

## 📖 Table of Contents
1. 📜[Prerequisites](#prerequisites)
2. 🖥️[Environment Setup](#environment-setup)
3. 💳[Wallet Configuration](#wallet-configuration)
4. 🚀[Compilation and Deployment](#compilation-and-deployment)
5. 🕵🏻[Testing and Execution](#testing-and-execution)
6. 🩺[Troubleshooting](#troubleshooting)

## 📝 Prerequisites
   Before getting started, make sure you have the following installed on your system:

### 1. Installing Rust 🦀:
- ### For Linux 🐧, macOS 🍎 Systems:
  If you using macOS, Linux, or any other Unix-like system:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- ### Windows 🤡:
  Download [Rust](https://www.rust-lang.org/tools/install) and run `rustup-init.exe`.

- ### Install the wasm32 target:
  After installing Rust, add the `wasm32-unknown-unknown` target:
```bash
rustup target add wasm32-unknown-unknown
```

## 2. Install Stellar CLI 📡:
- There are a few ways to install the [latest version](https://github.com/stellar/stellar-cli/releases) of Stellar CLI.
- Rust allows you to use `cargo` command in the terminal to install the Stellar CLI.

- ### Install with cargo 📦:
```sh
cargo install --locked stellar-cli --features opt
```

- ### Install with Homebrew (macOS, Linux):
```sh
brew install stellar-cli
```

## Environment Setup 🛠️ :

- 1. Clone the repository 🗂️:
   ```bash
   git clone https://github.com/<username>/Revo-Contracts.git
   cd ./Revo-Contracts
   ```

- 2. Build Project 👷‍♂️: 
   ```bash
   stellar contract build
   ```

-  Run the Tests 🕵️:
   ```bash
   cargo test
   ```

## Wallet Configuration 🤑 :
1. Install the Stellar Wallet (e.g., [Freighter Wallet](https://www.freighter.app/)).
2. Create a wallet and save the secret keys 🔑 securely.
3. Connect wallet to the Stellar test network.
   
## Compilation and Deployment 🚀 :

### 1. Build contract 👷‍♂️:
Once you have fully set up the contract in your local environment, installed all the necessary tools.
Then first step is to compile the contract and generate the `.wasm` file, 
which can be done using the following command:
```bash
stellar contract build
```

### 2. Install contract 📄:
- Before deploying the contract, you must first install it. This means uploading a version of your code to the Stellar network, which you can later use for deployment.
- When you execute the following command with the parameters specific to your local environment, it will return a hash. You will need to save this hash, as it will be required in the next step.

### macOS/Linux 💿:
```bash
stellar contract install \
   --network <network> \
   --source <source_account> \
   --wasm <path_to_wasm_file>
```
- ### Windows (PowerShell) 🤡:
```bash
stellar contract install `
   --network <network> `
   --source <source_account> `
   --wasm <path_to_wasm_file>
```
Where:
- `<network>` is the name of the network you are working.
- `<source_account>` is the account from which the installation will be made (you need to provide your own account).
- `<path_to_wasm_file>` is the path to the `.wasm` file generated when compiling the contract."

Response:
```
d36cd70c3b9c999e172ecc4648e616d9a49fd5dbbae8c28bef0b90bbb32fc762
```

### 3. Deploy contract 🧨:
- To deploy the contract, use the output of the previous command as this command's input parameter.
- Executing this command provides a hash, the contract ID. Use it to query platforms like [Stellar Expert](https://stellar.expert/explorer/testnet) and monitor contract interactions.

- ### macOS/Linux 💿:
```bash
stellar contract deploy \
   --wasm-hash <wasm_hash> \
   --source <source_account> \
   --network <network>
```
### Example💡:
- Assume the following values :
   <wasm_hash>: abc123def456ghi789jkl000mnopqr123stuvwx
   <source_account>: GBZXN7PIRZGNWCXXFYU7KYWXX4BXZUYHZO5QUEMKRHLUVLYN53WVFG3E
   <network>: testnet

```bash   
stellar contract deploy \
   --wasm-hash abc123def456ghi789jkl000mnopqr123stuvwx \
   --source GBZXN7PIRZGNWCXXFYU7KYWXX4BXZUYHZO5QUEMKRHLUVLYN53WVFG3E \
   --network testnet  
```

- ### Windows 🤡:
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

## Testing and Execution 🔬:
   After compiling and deploying your smart contracts, follow these steps to test and execute them efficiently:
   - Unit Tests
     Use cargo test to validate individual contract functions.
   ```bash
   cargo test
   ```
   - Fix any errors and re-run the tests.

### Simulating Transactions 🤖:
- Simulate contract calls to ensure correctness:
```bash
stellar contract invoke \
   --contract-id <contract_id> \
   --source <source_account> \
   --network <network> \
   --function <function_name> \
   --args <function_arguments>
   ```
  Where:
- <contract_id> is the deployed contract ID.
- <function_name> is the function being tested.

## Troubleshooting 🩺:
### Common Issues and Fixes🤔:
1. 🦀Rust Installation Issues:
   - Ensure `cargo` is in your system PATH.

2. 📡Stellar CLI Errors:
   - Verify the version compatibility of the Stellar CLI.
   - Use the --help flag to get details of commands:
   ```bash
    stellar --help
   ```
3. 💸Wallet Connectivity:
   - Double-check network configuration (testnet/mainnet).
---
