use soroban_sdk::{contracttype, Address, Env, String, Vec};
// use crate::datatypes::{Error};

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Rating {
    pub buyer: Address,
    pub rating: u32,
    pub weight: u32,
    pub feedback: Option<String>,
}

pub const MIN_RATING: u32 = 0;
pub const MAX_RATING: u32 = 5;

pub fn rate_seller_system(
    env: Env,
    seller: Address,
    buyer: Address,
    rating: u32,
    weight: u32,
    feedback: Option<String>,
) -> bool {
    if rating < MIN_RATING || rating > MAX_RATING {
        panic!("rating value must be in rang");
    }

    // let key = (seller.clone(), buyer.clone());

    // let has_rated: Option<bool> = env.storage().instance().get(&key);
    // if has_rated.is_some() && has_rated.unwrap() {
    //     panic!("");
    // }

    let seller_rating = Rating {
        buyer,
        rating,
        weight,
        feedback,
    };

    let mut ratings: Vec<Rating> = env
        .storage()
        .instance()
        .get(&seller)
        .unwrap_or(Vec::new(&env));
    ratings.push_back(seller_rating);
    env.storage().instance().set(&seller, &ratings);

    // env.storage().instance().set(&key, &true);

    true
}

pub fn update_weighted_rating(env: Env, seller: Address, rating: u32, weight: u32) {
    let (mut total_weighted_rating, mut total_weight): (u32, u32) =
        env.storage().instance().get(&seller).unwrap_or((0, 0));

    total_weighted_rating += rating * weight;
    total_weight += weight;

    env.storage()
        .instance()
        .set(&seller, &(total_weighted_rating, total_weight));
}

pub fn calculate_weighted_rating(env: Env, seller: Address) -> f32 {
    let (total_weighted_rating, total_weight): (u32, u32) =
        env.storage().instance().get(&seller).unwrap_or((0, 0));

    if total_weight == 0 {
        panic!("No rating availble");
    }

    (total_weighted_rating / total_weight) as f32
}
