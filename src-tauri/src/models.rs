use serde::{Deserialize, Serialize};
use blind_rsa_signatures::{KeyPair, Signature, PublicKey};

#[derive(Serialize, Deserialize)]
pub struct BlindSignatureData {
    pub blind_sig: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Ballot {
    pub id: String,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
    pub election_id: String,
    pub candidate_index: usize,
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
    pub public_key: Option<PublicKey>,
    pub signature: Option<Signature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Election {
    pub id: String,
    pub candidates: Vec<String>,
    pub vote_counts: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct BlindingResultData {
    pub blind_msg: Vec<u8>,
    pub id_number: String,
}