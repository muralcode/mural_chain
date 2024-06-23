/*Author: Lerato Mokoena
  Company: Arithoptix Pty Ltd (Mural Code).
  Date: June 23, 2024
  Description: Account creation
 */
use anyhow::Result;
use web3::types::H160;
use secp256k1::{SecretKey, PublicKey};

pub struct Account {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl Account {
    pub fn create_new_account() -> Result<Self> {
        let (secret_key, public_key) = super::key_management::generate_keypair();
        Ok(Account { secret_key, public_key })
    }

    pub async fn get_address(&self) -> Result<H160> {
        super::address::compute_address(&self.public_key).await
    }
}
