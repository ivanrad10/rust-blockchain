use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Wallet {
    public_key: PublicKey,
    private_key: SecretKey,
    balance: u64,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let private_key = SecretKey::new(& mut rng);
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        Wallet {
            public_key,
            private_key,
            balance: 0,
        }
    }

    pub fn get_public_key(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    fn sign_transaction(&self, tx_data: &str) -> String {
        let secp = Secp256k1::new();
        let message = secp256k1::Message::from_hashed_data::<secp256k1::hashes::sha256::Hash>(tx_data.as_bytes());
    
        let signature = secp.sign_ecdsa(&message, &self.private_key);
        hex::encode(signature.serialize_compact())
    }
}