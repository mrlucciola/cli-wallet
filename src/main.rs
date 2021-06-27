use anyhow::Result;
use web3::types::Address;
pub mod wallet_lib;
use crate::wallet_lib::{create_txn_obj, sign_and_send};
use std::str::FromStr;
use tokio;

const URL: &str = "https://eth-ropsten.alchemyapi.io/v2/xxxxxxxxx";

#[tokio::main]
async fn main() -> Result<()> {
    let keypair = wallet_lib::create_keypair()?;
    println!("\nkeypair: {:?}", keypair);
    
    let web3 = wallet_lib::establish_web3_connection(URL)?;
    println!("\nweb3: {:?}", web3);
    
    let to_addr = Address::from_str("0xaaa5b9e6c589642f98a1cda99b9d024b8407285a")?;
    println!("\nto_addr: {:?}", to_addr);
    
    let tx_obj = create_txn_obj(to_addr, 7)?;
    println!("\ntx_obj: {:?}", tx_obj);

    let result = sign_and_send(web3.clone(), tx_obj, keypair.0).await?;
    
    println!("\nresult: {}", result);

    Ok(())
}
