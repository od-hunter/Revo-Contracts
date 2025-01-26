use soroban_sdk::{contracttype, Address, BytesN, Env, Symbol};

#[contracttype]
pub struct NFTMetadata {
    pub buyer: Address,
    pub seller: Address,
    pub amount: u64,
    pub product: BytesN<32>,
    pub timestamp: u64,
}

pub fn mint_nft(env: &Env, buyer: &Address, tx_id: BytesN<32>, seller: &Address, amount: u64, product: &BytesN<32>) {
    if buyer == seller {
        panic!("Buyer and seller cannot be the same address");
    }

    let timestamp = env.ledger().timestamp();
    let metadata = NFTMetadata {
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount,
        product: product.clone(),
        timestamp,
    };

    // Store metadata in contract instance storage
    env.storage().instance().set(&tx_id, &metadata);

    // Emit event for tracking the mint operation
    env.events().publish(
        (Symbol::new(env, "nft_minted"), tx_id.clone()), // Event key
        (buyer.clone(), seller.clone(), amount, product.clone(), timestamp), // Event data
    );
}
