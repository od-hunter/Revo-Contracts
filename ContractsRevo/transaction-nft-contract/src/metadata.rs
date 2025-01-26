use crate::mint::NFTMetadata;
use soroban_sdk::{BytesN, Env};

pub fn get_metadata(env: &Env, tx_id: &BytesN<32>) -> Option<NFTMetadata> {
    env.storage().instance().get(tx_id)
}
