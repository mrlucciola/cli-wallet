use anyhow::Result;

pub mod wallet_lib;
fn main() -> Result<()> {
    let keypair = wallet_lib::create_keypair();
    println!("keypair: {:?}", keypair);

    Ok(())
}
