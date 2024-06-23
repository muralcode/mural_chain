/*Author: Lerato Mokoena
  Company: Arithoptix Pty Ltd (Mural Code).
  Date: June 23, 2024
  Description: Key pair gen
 */
use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    secp.generate_keypair(&mut rng)
}
