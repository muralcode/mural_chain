/*Author: Lerato Mokoena
  Company: Arithoptix Pty Ltd (Mural Code).
  Date: June 23, 2024
  Description: Transactions viewing
 */
use anyhow::Result;
use web3::transports::Http;
use web3::types::H160;
use web3::Web3;

pub async fn view_transactions(web3: &Web3<Http>, address: H160) -> Result<Vec<String>> {
    let block_number = web3.eth().block_number().await?;
    let block = web3.eth().block_with_txs(block_number.into()).await?.unwrap();

    let mut tx_hashes = Vec::new();
    for tx in block.transactions {
        if tx.from == address || tx.to == Some(address) {
            tx_hashes.push(tx.hash.to_string());
        }
    }

    Ok(tx_hashes)
}
