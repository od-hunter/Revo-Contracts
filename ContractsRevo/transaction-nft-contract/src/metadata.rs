use soroban_sdk::{BytesN, Env};

use crate::mint::NFTMetadata;

pub fn get_metadata(env: &Env, tx_id: &BytesN<32>) -> Option<NFTMetadata> {
    env.storage().instance().get(tx_id)
}
