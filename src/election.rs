use crate::voter::Voter;
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};
use std::collections::HashMap;

pub struct Election {
    voters: Vec<Voter>,
    votes: HashMap<String, Vec<u8>>,
}

impl Election {
    pub fn new() -> Self {
        Election {
            voters: Vec::new(),
            votes: HashMap::new(),
        }
    }

    pub fn register_voter(&mut self, voter: Voter) {
        self.voters.push(voter);
    }

    pub fn cast_vote(&mut self, voter: &Voter, vote: String) {
        let public_key = &voter.public_key;
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let encrypted_vote = public_key
            .encrypt(&mut rand::thread_rng(), padding, vote.as_bytes())
            .expect("failed to encrypt");
        let public_key_pem = public_key
            .to_pkcs1_pem()
            .expect("failed to encode public key");
        self.votes.insert(public_key_pem, encrypted_vote);
    }

    pub fn verify_votes(&self) -> HashMap<String, String> {
        let mut verified_votes = HashMap::new();

        for voter in &self.voters {
            let private_key = &voter.private_key;
            let padding = PaddingScheme::new_pkcs1v15_encrypt();
            let public_key_pem = voter
                .public_key
                .to_pkcs1_pem()
                .expect("failed to encode public key");
            if let Some(encrypted_vote) = self.votes.get(&public_key_pem) {
                let decrypted_vote = private_key
                    .decrypt(padding, encrypted_vote)
                    .expect("failed to decrypt");
                verified_votes.insert(
                    public_key_pem,
                    String::from_utf8(decrypted_vote).expect("failed to convert to string"),
                );
            }
        }

        verified_votes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voter::Voter;

    #[test]
    fn test_register_voter() {
        let mut election = Election::new();
        let voter = Voter::new();
        election.register_voter(voter.clone());
        assert_eq!(election.voters.len(), 1);
    }

    #[test]
    fn test_cast_and_verify_vote() {
        let mut election = Election::new();
        let voter = Voter::new();
        election.register_voter(voter.clone());

        election.cast_vote(&voter, "Candidate A".to_string());
        let verified_votes = election.verify_votes();

        assert_eq!(verified_votes.len(), 1);
        assert_eq!(verified_votes.values().next().unwrap(), "Candidate A");
    }
}
