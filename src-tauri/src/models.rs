use serde::{Deserialize, Serialize};
use blind_rsa_signatures::{KeyPair, Signature, PublicKey};

#[derive(Serialize, Deserialize)]
pub struct BlindSignatureData {
    pub blind_sig: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Vote {
    pub election: String,
    pub candidate: String,
    pub user_public_key: String, // PEM format
    pub user_signature: Vec<u8>, // Signature on the vote
    pub server_signature: Vec<u8>, // Server's signature on user's public key
}

pub struct SignedKey {
    pub keypair: KeyPair,
    pub sig: Signature,
}


#[derive(Deserialize, Serialize, Clone)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub dob: String,
    pub driv_lic: String,
    pub public_key: PublicKey,
    pub signature: Option<Signature>,
}
