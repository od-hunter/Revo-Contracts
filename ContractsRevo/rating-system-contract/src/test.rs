#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as TestAddress, Env};

#[test]
fn test_rate_seller() {
    let env = Env::default();
    let contract_id = env.register(RatingSytemContract, ());
    let client = RatingSytemContractClient::new(&env, &contract_id);
    let seller_address = <Address>::generate(&env);
    let buyer_address = <Address>::generate(&env);  
    let rating = 3u32;
    let weight = 1u32;  
    

    client.rate_seller(&seller_address, &buyer_address, &rating, &weight, &None);

    env.as_contract(&contract_id, || {
        let (total_weighted_rating,total_weight): (u32, u32) = env.storage().instance().get(&seller_address).unwrap();
        assert_eq!(total_weighted_rating, rating);
        assert_eq!(total_weight, weight);
    });


}
