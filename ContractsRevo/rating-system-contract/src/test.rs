#![cfg(test)]

use crate::rating::Rating;
use super::*;
use soroban_sdk::{testutils::Address as TestAddress, Env, Vec};

#[test]
#[should_panic(expected = "Buyer and seller cannot be the same address")]
fn test_rate_seller_panic_same_buyer_seller() {
    let env = Env::default();
    let contract_id = env.register(RatingSytemContract, ());
    let client = RatingSytemContractClient::new(&env, &contract_id);
    let seller_address = <Address>::generate(&env);
    let rating = 3u32;  
    let weight = 2u32;

    client.rate_seller(&seller_address, &seller_address, &rating, &weight, &None);
}

#[test]
#[should_panic(expected = "Rating must be between 1 and 5")]
fn test_rate_seller_panic_rating_out_of_range() {
    let env = Env::default();
    let contract_id = env.register(RatingSytemContract, ());
    let client = RatingSytemContractClient::new(&env, &contract_id);
    let seller_address = <Address>::generate(&env);
    let buyer_address = <Address>::generate(&env);  
    let weight = 2u32;

    client.rate_seller(&seller_address, &buyer_address, &10, &weight, &None);
}

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

    // Helper function to verify storage
    fn assert_storage(
        env: &Env,
        contract_id: &Address,
        weighted_key: &DataKey,
        rating_key: &DataKey,
        expected_ratings: u32,
        expected_weighted_rating: u32,
        expected_total_weight: u32,
    ) {
        env.as_contract(contract_id, || {
            let (total_weighted_rating, total_weight): (u32, u32) = env
                .storage()
                .instance()
                .get(weighted_key)
                .expect("Weighted rating key not found");

            let ratings: Vec<Rating> = env
                .storage()
                .instance()
                .get(rating_key)
                .expect("Rating key not found");

            assert_eq!(ratings.len(), expected_ratings);
            assert_eq!(total_weighted_rating, expected_weighted_rating);
            assert_eq!(total_weight, expected_total_weight);
        });
    }

    // First rating submission
    client.rate_seller(&seller_address, &buyer_address, &rating, &weight, &None);
    assert_storage(&env, &contract_id, &weighted_key, &rating_key, 1, rating * weight, weight);

   // Second and third ratings by the same buyer
   client.rate_seller(&seller_address, &buyer_address, &rating, &weight, &None);
   client.rate_seller(&seller_address, &buyer_address, &rating, &weight, &None);
   assert_storage(
       &env,
       &contract_id,
       &weighted_key,
       &rating_key,
       3,
       (rating * weight) * 3,
       weight * 3,
   );

   // Edge case: Rating with zero weight
   client.rate_seller(&seller_address, &buyer_address, &rating, &0, &None);
   assert_storage(
       &env,
       &contract_id,
       &weighted_key,
       &rating_key,
       4,
       (rating * weight) * 3,
       weight * 3,
   );
}

