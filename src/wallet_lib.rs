use anyhow::Result;
use secp256k1::{PublicKey, Secp256k1, SecretKey, rand::{SeedableRng, rngs::StdRng}};

pub fn create_keypair() -> Result<(SecretKey, PublicKey)> {
    let secp = Secp256k1::new();
    let mut rng = StdRng::seed_from_u64(6);
    Ok(secp.generate_keypair(&mut rng))
}