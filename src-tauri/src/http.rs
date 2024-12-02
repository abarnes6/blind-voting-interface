use blind_rsa_signatures::{BlindSignature, BlindingResult, Options, PublicKey};

use crate::{crypto::get_keypair, models::{BlindSignatureData, BlindingResultData, Election, SignedKey, Vote}};

pub async fn get_server_public_key() -> PublicKey {
    let response = reqwest::get("http://127.0.0.1:7878/public-key")
        .await
        .expect("Failed to fetch server public key");
    let pem = response
        .text()
        .await
        .expect("Failed to read public key response");
    PublicKey::from_pem(&pem).expect("Failed to parse server public key")
}

pub async fn fetch_elections() -> Vec<Election> {
    let response = reqwest::get("http://127.0.0.1:7878/elections")
        .await
        .expect("Failed to fetch elections");
    response
        .json::<Vec<Election>>()
        .await
        .expect("Failed to parse elections")
}

pub async fn fetch_blind_signature(brd: BlindingResultData, br: BlindingResult, msg: Vec<u8>, options: Options) -> Result<SignedKey, String> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:7878/blind-sign")
        .json(&brd)
        .send()
        .await
        .expect("Failed to send blind signature request");

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap();
        println!("Failed to get blind signature: {}", error_text);
        return Err(error_text);
    }

    // Handle the response
    let blind_sig_data: BlindSignatureData = response
        .json()
        .await
        .expect("Failed to deserialize server response");

    let blind_sig = BlindSignature::from(blind_sig_data.blind_sig);
    let server_pk = get_server_public_key().await;
    let sig = server_pk
        .finalize(&blind_sig, &br.secret, br.msg_randomizer.clone(), &msg, &options)
        .expect("Failed to finalize signature");

    let signed_key = SignedKey {
        keypair: get_keypair(),
        sig,
    };

    
    assert!(signed_key
        .sig
        .verify(&server_pk,
                None,
                &msg,
                &options)
        .is_ok());

    Ok(signed_key)
}

pub async fn submit_vote(vote: Vote) -> Result<bool, String> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:7878/submit-vote")
        .json(&vote)
        .send()
        .await
        .expect("Failed to submit vote");

    if response.status().is_success() {
        return Ok(true);
    } else {
        let error_text = response.text().await.unwrap();
        return Err(format!("Failed to submit vote: {}", error_text));
    }
}