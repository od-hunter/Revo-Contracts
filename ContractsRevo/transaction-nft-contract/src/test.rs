#![cfg(test)]
extern crate std;
use super::*;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger},
    Address, BytesN, Env, IntoVal,
};

#[test]
fn test_metadata_attachment() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().set_timestamp(123456789);

    let contract_id = env.register(TransactionNFTContract, ());
    let client = TransactionNFTContractClient::new(&env, &contract_id);

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let amount = 100;
    let product = BytesN::from_array(&env, &[0; 32]);

    // Mint the NFT
    let tx_id = client.mint_nft(&buyer, &seller, &amount, &product);

    // Verify authorizations
    assert_eq!(
        env.auths(),
        std::vec![
            (
                buyer.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_id.clone(),
                        symbol_short!("mint_nft"),
                        (buyer.clone(), seller.clone(), 100_u64, product.clone()).into_val(&env),
                    )),
                    sub_invocations: std::vec![]
                }
            ),
            (
                seller.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_id.clone(),
                        symbol_short!("mint_nft"),
                        (buyer.clone(), seller.clone(), 100_u64, product.clone()).into_val(&env),
                    )),
                    sub_invocations: std::vec![]
                }
            ),
        ]
    );

    // Retrieve the metadata
    let metadata = client.get_nft_metadata(&tx_id).unwrap();

    // Verify purchase details storage
    assert_eq!(metadata.amount, amount);
    assert_eq!(metadata.product, product);

    // Test buyer-seller information
    assert_eq!(metadata.buyer, buyer);
    assert_eq!(metadata.seller, seller);

    // Validate data completeness
    assert!(metadata.timestamp > 0);
}

// #[test]
