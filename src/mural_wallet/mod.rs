/*Author: Lerato Mokoena
  Company: Arithoptix Pty Ltd (Mural Code).
  Date: June 15, 2024
  Description: Central module file to export the submodules
 */

pub mod key_management;
pub mod address;
pub mod account;
pub mod hardware_wallet;
pub mod transactions;

use anyhow::Result;
use secp256k1::{PublicKey, SecretKey};
use web3::types::{H160, U256};
use web3::transports::Http;
use web3::Web3;

pub struct EthWallet {
    secret_key: SecretKey,
    public_key: PublicKey,
    web3: Web3<Http>,
}

impl EthWallet {
    pub fn new () -> Self {
        let (secret_key, public_key) = key_management::generate_keypair();
        let http = Http::new("https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID").expect("Invalid HTTP endpoint");
        let web3 = Web3::new(http);

        EthWallet {
            secret_key,
            public_key,
            web3,
        }
    }

    pub fn get_secret_key(&self) -> Result<SecretKey> {
        Ok(self.secret_key.clone())
    }

    pub fn get_public_key(&self) -> Result<PublicKey> {
        Ok(self.public_key.clone())
    }

    pub async fn get_address(&self) -> Result<H160> {
        address::compute_address(&self.public_key).await
    }

    pub async fn view_balance(&self, address: H160) -> Result<U256> {
        let balance = self.web3.eth().balance(address, None).await?;
        Ok(balance)
    }

    pub async fn buy_eth(&self, address: H160, amount: f64) -> Result<H160> {
        // Placeholder for actual implementation
        transactions::buy_eth(&self.web3, address, amount).await
    }

    pub async fn view_transactions(&self, address: H160) -> Result<Vec<String>> {
        transactions::view_transactions(&self.web3, address).await
    }
}




