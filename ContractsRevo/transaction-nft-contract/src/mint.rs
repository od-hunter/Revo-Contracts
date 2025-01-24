use soroban_sdk::{contracttype, Address, BytesN, Env};

#[contracttype]
pub struct NFTMetadata {
    pub buyer: Address,
    pub seller: Address,
    pub amount: u64,
    pub product: BytesN<32>,
    pub timestamp: u64,
}

pub fn mint_nft(env: &Env, buyer: &Address, tx_id: BytesN<32>, seller: &Address, amount: u64, product: &BytesN<32>) {
    let timestamp = env.ledger().timestamp();
    let metadata = NFTMetadata {
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount,
        product: product.clone(),
        timestamp,
    };

    env.storage().instance().set(&tx_id, &metadata);
}
