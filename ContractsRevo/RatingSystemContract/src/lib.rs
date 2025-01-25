#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol};

use rating::{rate_seller_system, update_weighted_rating};
use reputation::{add_reputation_score_history, reputation_score_calculate};

mod history;
mod rating;
mod reputation;

#[contract]
pub struct RatingSytemContract;

#[contractimpl]
impl RatingSytemContract {
    pub fn rate_seller(
        env: Env,
        seller: Address,
        buyer: Address,
        rating: u32,
        weight: u32,
        feedback: Option<String>,
    ) {
        if buyer == seller {
            panic!("Buyer and seller cannot be the same address");
        }

        if !(1..=5).contains(&rating) {
            panic!("Rating must be between 1 and 5");
        }

        if rate_seller_system(
            env.clone(),
            seller.clone(),
            buyer.clone(),
            rating,
            weight,
            feedback,
        ) {
            update_weighted_rating(env.clone(), seller.clone(), rating, weight);
        }

        env.events().publish(
            (Symbol::new(&env, "buyer_rate_seller"), seller.clone()),
            rating,
        );
    }

    pub fn seller_reputation_score(env: Env, seller: Address) -> u32 {
        if seller.to_string().len() == 0 {
            panic!("seller address is invalid");
        }

        let reputation_score = reputation_score_calculate(env.clone(), seller.clone());
        add_reputation_score_history(env.clone(), seller.clone(), reputation_score.clone());

        env.events().publish(
            (Symbol::new(&env, "reputation_score_updated"), seller),
            reputation_score,
        );
        reputation_score
    }
}
