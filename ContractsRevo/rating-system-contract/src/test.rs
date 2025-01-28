#![cfg(test)]

use crate::rating::Rating;
use super::*;
use soroban_sdk::{testutils::Address as TestAddress, Env, Vec};

#[test]
fn test_rate_seller() {
    let env = Env::default();
    let contract_id = env.register(RatingSytemContract, ());
    let client = RatingSytemContractClient::new(&env, &contract_id);
    let seller_address = <Address>::generate(&env);
    let buyer_address = <Address>::generate(&env);  
    let rating = 3u32;  
    let weight = 2u32;  
    let weighted_key = DataKey::WeightedRating(seller_address.clone());
    let rating_key = DataKey::Rating(seller_address.clone());

    client.rate_seller(&seller_address, &buyer_address, &rating, &weight, &None);
    
    env.as_contract(&contract_id, || {
        let (total_weighted_rating,total_weight): (u32, u32) = env.storage().instance().get(&weighted_key).unwrap();
        let ratings: Vec<Rating> = env.storage().instance().get(&rating_key).unwrap();

        assert_eq!(ratings.len(), 1);
        assert_eq!(total_weighted_rating, rating*weight);
        assert_eq!(total_weight, weight);
    });

    client.rate_seller(&seller_address, &buyer_address, &rating, &weight, &None);
    client.rate_seller(&seller_address, &buyer_address, &rating, &weight, &None);

    env.as_contract(&contract_id, || {
        let (total_weighted_rating,total_weight): (u32, u32) = env.storage().instance().get(&weighted_key).unwrap();
        let ratings: Vec<Rating> = env.storage().instance().get(&rating_key).unwrap();

        assert_eq!(ratings.len(), 3);
        assert_eq!(total_weighted_rating, (rating*weight)*3);
        assert_eq!(total_weight, weight * 3);
    });
}
