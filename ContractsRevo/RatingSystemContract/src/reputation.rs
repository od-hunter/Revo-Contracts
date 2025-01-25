use soroban_sdk::{contracttype, Address, Env, Vec};

use crate::history;

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct ReputationRecord {
    pub score: u32,
    pub timestamp: u64,
}

pub fn reputation_score_calculate(env: Env, seller: Address) -> u32 {
    let weighted_rating = crate::rating::calculate_weighted_rating(env, seller);
    match weighted_rating {
        0.0..=1.0 => 1,
        2.0..=2.0 => 2,
        3.0..=3.0 => 3,
        4.0..=4.0 => 4,
        _ => 5,
    }
}

pub fn add_reputation_score_history(env: Env, seller: Address, score: u32) {
    let timestamp = env.ledger().timestamp();

    let mut reputation_history: Vec<ReputationRecord> =
        history::get_reputation_history(env.clone(), seller.clone());

    let new_record = ReputationRecord { score, timestamp };

    reputation_history.push_back(new_record);

    env.storage().instance().set(&seller, &reputation_history);
}
