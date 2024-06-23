/*Author: Lerato Mokoena
  Company: Arithoptix Pty Ltd (Mural Code).
  Date: June 23, 2024
  Description: Address
 */
use anyhow::Result;
use secp256k1::PublicKey;
use web3::signing::keccak256;
use web3::types::H160;

pub async fn compute_address(public_key: &PublicKey) -> Result<H160> {
    let public_key_bytes = &public_key.serialize_uncompressed()[1..];
    let address_bytes = keccak256(public_key_bytes);
    Ok(H160::from_slice(&address_bytes[12..]))
}
