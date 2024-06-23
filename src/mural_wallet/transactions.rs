/*Author: Lerato Mokoena
  Company: Arurea Murals Pty Ltd (Mural Code).
  Date: June 23, 2024
  Description: Transactions
 */
use anyhow::Result;
use web3::transports::Http;
use web3::types::{TransactionRequest, H160, U256};
use web3::Web3;

pub async fn buy_eth(web3: &Web3<Http>, address: H160, amount: f64) -> Result<H160> {
    // Placeholder for actual buy ETH transaction implementation
    let tx = TransactionRequest {
        from: address,
        to: Some(address),
        gas: None,
        gas_price: None,
        value: Some(U256::from((amount * 1e18) as u64)), // Convert ETH to Wei
        data: None,
        nonce: None,
        condition: None,
        transaction_type: todo!(),
        access_list: todo!(),
        max_fee_per_gas: todo!(),
        max_priority_fee_per_gas: todo!(),
    };

    let tx_hash = web3.eth().send_transaction(tx).await?;
    Ok(tx_hash.into())
}

pub async fn view_transactions(web3: &Web3<Http>, address: H160) -> Result<Vec<String>> {
    // Placeholder for actual transaction fetching implementation
    Ok(vec!["tx_hash_1".to_string(), "tx_hash_2".to_string()])
}
