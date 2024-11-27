extern crate blind_rsa_signatures;
extern crate rand;

use blind_rsa_signatures::{reexports::rsa::Pkcs1v15Sign, BlindSignature, KeyPair, Options, PublicKey, SecretKey, Signature};
use rand::thread_rng;
use sha2::{Digest, Sha256};
use std::{fs};
use crate::models::{BlindSignatureData, Profile, SignedKey, Vote};

// Function to get the server's public key
async fn get_server_public_key() -> PublicKey {
    // For simplicity, assume the server's public key is available at this endpoint
    let response = reqwest::get("http://127.0.0.1:7878/public-key")
        .await
        .expect("Failed to fetch server public key");
    let pem = response
        .text()
        .await
        .expect("Failed to read public key response");
    PublicKey::from_pem(&pem).expect("Failed to parse server public key")
}

// Keypair management functions
fn generate_keypair() -> KeyPair {
    let rng = &mut thread_rng();
    let kp = KeyPair::generate(rng, 2048).unwrap();
    write_keypair(&kp);
    kp
}

fn write_keypair(kp: &KeyPair) {
    fs::write(
        "pkey",
        kp.pk.to_pem().expect("Error converting pubkey to PEM."),
    )
        .expect("Error writing public key to disk.");
    fs::write(
        "skey",
        kp.sk.to_pem().expect("Error converting seckey to PEM."),
    )
        .expect("Error writing secret key to disk.");
}

fn keypair_exists() -> bool {
    secret_exists() && public_exists()
}

fn secret_exists() -> bool {
    fs::metadata("skey").is_ok()
}

fn public_exists() -> bool {
    fs::metadata("pkey").is_ok()
}

fn get_secret() -> SecretKey {
    if !keypair_exists() {
        generate_keypair();
    }
    let key = fs::read_to_string("skey").expect("Failed to read secret key");
    SecretKey::from_pem(&key).expect("Failed to parse secret key")
}

fn get_public() -> PublicKey {
    if !keypair_exists() {
        generate_keypair();
    }
    let key = fs::read_to_string("pkey").expect("Failed to read public key");
    PublicKey::from_pem(&key).expect("Failed to parse public key")
}

pub fn get_keypair() -> KeyPair {
    KeyPair::new(get_public(), get_secret())
}

pub async fn request_ballot(profile: Profile, keypair: KeyPair) -> Signature {
    let options = Options::default();
    let rng = &mut thread_rng();

    let local_pubkey = keypair.pk.to_der().unwrap();
    let server_pubkey = get_server_public_key().await;

    let blinded_local_pubkey = server_pubkey
        .blind(rng, &local_pubkey, true, &options)
        .expect("Failed to blind the message");
    
    let profile_to_send = Profile {
        first_name: profile.first_name,
        last_name: profile.last_name,
        address: profile.address,
        dob: profile.dob,
        driv_lic: profile.driv_lic,
        public_key: PublicKey::from_der(&blinded_local_pubkey.blind_msg.0).expect("Failed to parse blinded public key"),
        signature: None,
    };

    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:7878/blind-sign")
        .json(&profile_to_send)
        .send()
        .await
        .expect("Failed to send request");

    let blind_sig_data: BlindSignatureData = response
        .json()
        .await
        .expect("Failed to deserialize server response");

    let blind_sig = BlindSignature::from(blind_sig_data.blind_sig);

    let sig = server_pubkey
        .finalize(
            &blind_sig,
            &blinded_local_pubkey.secret,
            blinded_local_pubkey.msg_randomizer,
            &local_pubkey,
            &options,
        )
        .expect("Failed to finalize signature");

    let signed_key = SignedKey { keypair, sig };

    assert!(signed_key
        .sig
        .verify(&server_pubkey, blinded_local_pubkey.msg_randomizer, &local_pubkey, &options)
        .is_ok());

    return signed_key.sig;
}

pub async fn submit_ballot(election: &str, candidate: &str, keypair: KeyPair, sig: Signature) -> bool {
    let vote_data = format!("{}:{}", election, candidate);
    let hashed_vote = Sha256::digest(vote_data.as_bytes());
    let user_signature = keypair.sk.0.sign(Pkcs1v15Sign::new::<Sha256>(), &hashed_vote).expect("Failed to sign");
    let vote = Vote {
        election: election.to_string(),
        candidate: candidate.to_string(),
        user_public_key: keypair.pk.to_pem().expect("Failed to convert public key to PEM"),
        user_signature: user_signature.to_vec(),
        server_signature: sig.to_vec(),
    };

    let client = reqwest::Client::new();
    // Submit the vote
    let response = client
        .post("http://127.0.0.1:7878/submit-vote")
        .json(&vote)
        .send()
        .await
        .expect("Failed to submit vote");

    return response.status().is_success();
}