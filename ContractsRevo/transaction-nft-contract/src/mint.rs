use soroban_sdk::{contracttype, Address, BytesN, Env, Symbol};

#[contracttype]
pub struct NFTMetadata {
    pub buyer: Address,
    pub seller: Address,
    pub amount: u64,
    pub product: BytesN<32>,
    pub timestamp: u64,
}

pub fn mint_nft(
    env: &Env,
    buyer: &Address,
    tx_id: BytesN<32>,
    seller: &Address,
    amount: u64,
    product: &BytesN<32>,
) {
    // Fetch the ledger timestamp
    let timestamp = env.ledger().timestamp();

    // Validate timestamp (ensure it's a valid nonzero timestamp)
    if timestamp == 0 {
        panic!("Invalid ledger timestamp");
    }

    let metadata = NFTMetadata {
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount,
        product: product.clone(),
        timestamp,
    };

    // Store metadata in contract instance storage
    env.storage().instance().set(&tx_id, &metadata);

    // Emit an event for tracking the mint operation
    env.events().publish(
        (Symbol::new(env, "nft_minted"), tx_id.clone()), // Event key
        tx_id.clone(),                                   // Event data limited to tx_id only
    );
}
