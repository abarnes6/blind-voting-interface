extern crate blind_rsa_signatures;
extern crate rand;

use blind_rsa_signatures::{KeyPair, PublicKey, SecretKey};
use rand::thread_rng;
use std::fs;

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