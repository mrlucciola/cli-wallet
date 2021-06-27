use anyhow::Result;
use secp256k1::{PublicKey, Secp256k1, SecretKey, rand::{SeedableRng, rngs::StdRng}};
use web3::{Web3, transports::Http, types::{H160, H256, TransactionParameters, U256}};

pub fn create_keypair() -> Result<(SecretKey, PublicKey)> {
    let secp = Secp256k1::new();
    let mut rng = StdRng::seed_from_u64(6);
    Ok(secp.generate_keypair(&mut rng))
}

pub fn establish_web3_connection(url: &str) -> Result<Web3<Http>> {
    let transport = Http::new(url)?;
    Ok(web3::Web3::new(transport))
}

pub fn create_txn_obj(to: H160, value: usize) -> Result<TransactionParameters> {
    Ok(TransactionParameters{
        to: Some(to),
        value: U256::exp10(value),
        nonce: Some(U256([0,0,0,0])),
        chain_id: Some(1),
        ..Default::default()
    })
}

pub async fn sign_and_send(web3: Web3<Http>, tx_object: TransactionParameters, seckey: SecretKey) -> Result<H256> {
    let signed = web3.accounts().sign_transaction(tx_object, &seckey).await?;
    Ok(web3.eth().send_raw_transaction(signed.raw_transaction).await?)
}