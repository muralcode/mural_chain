/*Author: Lerato Mokoena
  Company: Arithoptix Pty Ltd (Mural Code).
  Date: June 23, 2024
  Description: Balance handling
 */
use anyhow::Result;
use web3::transports::Http;
use web3::types::H160;
use web3::Web3;
use web3::types::U256;

pub async fn view_balance(web3: &Web3<Http>, address: H160) -> Result<U256> {
    let balance = web3.eth().balance(address, None).await?;
    Ok(balance)
}
