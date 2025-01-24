use soroban_sdk::{Address, Bytes, BytesN, Env};
use soroban_sdk::xdr::ToXdr;

pub fn generate_transaction_proof(
    env: Env,
    buyer: Address,
    seller: Address,
    amount: u64,
    product: Bytes,
) -> BytesN<32> {
    // Convert Addresses to XDR format
    let buyer_xdr: Bytes = buyer.to_xdr(&env);
    let seller_xdr: Bytes = seller.to_xdr(&env);
    // Convert amount to bytes (big-endian)
    let amount_bytes = amount.to_be_bytes();
    // Create a single Bytes buffer to concatenate all data
    let mut data = Bytes::new(&env);
    data.append(&buyer_xdr);  // Append buyer address
    data.append(&seller_xdr); // Append seller address
    data.append(&Bytes::from_array(&env, &amount_bytes)); // Append amount
    data.append(&product);    // Append product bytes
    // Compute SHA-256 hash
    let hash = env.crypto().sha256(&data);
    // Convert hash result to BytesN<32>
    BytesN::from_array(&env, &hash.to_array())
}

