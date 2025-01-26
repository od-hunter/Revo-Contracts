use soroban_sdk::{Address, Bytes, BytesN, Env};
use soroban_sdk::xdr::ToXdr;

pub fn generate_transaction_proof(
    env: Env,
    buyer: Address,
    seller: Address,
    amount: u64,
    product: Bytes,
) -> BytesN<32> {
    let buyer_xdr: Bytes = buyer.to_xdr(&env);
    let seller_xdr: Bytes = seller.to_xdr(&env);
    let amount_bytes = amount.to_be_bytes();
    let timestamp_bytes = env.ledger().timestamp().to_be_bytes();
    let contract_address = env.current_contract_address();
    let nonce = env.crypto().random();

    let mut data = Bytes::new(&env);
    data.append(&buyer_xdr);
    data.append(&seller_xdr);
    data.append(&Bytes::from_array(&env, &amount_bytes));
    data.append(&product);
    data.append(&Bytes::from_array(&env, &timestamp_bytes));
    data.append(&contract_address.to_xdr(&env));
    data.append(&nonce.to_xdr(&env));

    let hash = env.crypto().sha256(&data);
    BytesN::from_array(&env, &hash.to_array())
}
