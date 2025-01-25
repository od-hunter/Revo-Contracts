use crate::{rating::Rating, reputation::ReputationRecord};
use soroban_sdk::{Address, Env, Vec};

pub fn get_rating_history(env: Env, seller: &Address) -> Vec<Rating> {
    env.storage().instance().get(&seller).unwrap()
}

pub fn get_reputation_history(env: Env, seller: Address) -> Vec<ReputationRecord> {
    env.storage()
        .instance()
        .get(&seller)
        .unwrap_or_else(|| Vec::new(&env))
}
