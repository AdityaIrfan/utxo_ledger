// use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::{RngCore}; // for trait bounds
use rand::rngs::OsRng; // note: from `rand`, NOT `rand_core`
// use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey
}

// #[derive(Serialize, Deserialize)]
// pub struct WalletData {
//     pub public_key: String,
//     pub private_key: String,
// }
impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);

        let signing_key: SigningKey = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();

        Self {
            signing_key,
            verifying_key
        }
    }

    // pub fn sign(&self, message: &[u8]) -> Signature {
    //     self.signing_key.sign(message)
    // }
    //
    // pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
    //     self.verifying_key.verify(message, signature).is_ok()
    // }
    //
    // pub fn get_serializable_data(&self) -> WalletData {
    //     WalletData{
    //         public_key: hex::encode(self.verifying_key.to_bytes()),
    //         private_key: hex::encode(self.signing_key.to_bytes()),
    //     }
    // }

    // pub fn from_serializable(data: &WalletData) -> Self {
    //     let sk_bytes = hex::decode(&data.private_key).expect("Invalid decode private key");
    //     let signing_key = SigningKey::from_bytes(&sk_bytes[..32].try_into().unwrap());
    //     let verifying_key = signing_key.verifying_key();
    //
    //     Self {
    //         signing_key,
    //         verifying_key
    //     }
    // }

    pub fn address(&self) -> String {
        let hash = Sha256::digest(&self.verifying_key.to_bytes());
        hex::encode(hash)
    }
}