use rand::rngs::OsRng;
use rsa::{
    pkcs1::DecodeRsaPrivateKey, pkcs1::EncodeRsaPrivateKey, PublicKey, RSAPrivateKey, RSAPublicKey,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Voter {
    pub public_key: RSAPublicKey,
    #[serde(skip)]
    private_key: RSAPrivateKey,
}

impl Voter {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);

        Voter {
            public_key,
            private_key,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voter_creation() {
        let voter = Voter::new();
        assert!(voter.public_key.n().bits() == 2048);
        assert!(voter.private_key.n().bits() == 2048);
    }
}
