#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

mod mint;
mod proof;
mod metadata;

#[contract]
pub struct TransactionNFTContract;

#[contractimpl]
impl TransactionNFTContract {
    pub fn mint_nft(env: Env, buyer: Address, seller: Address, amount: u64, product: BytesN<32>) -> BytesN<32> {
        let product_bytes: soroban_sdk::Bytes = product.clone().into();
        let tx_id = proof::generate_transaction_proof(env.clone(), buyer.clone(), seller.clone(), amount, product_bytes);
        mint::mint_nft(&env, &buyer, tx_id.clone(), &seller, amount, &product);
        tx_id
    }
}
