mod models;
mod crypto;
mod http;

use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use blind_rsa_signatures::reexports::rsa::Pkcs1v15Sign;
use blind_rsa_signatures::Options;
use blind_rsa_signatures::Signature;
use models::BlindingResultData;
use models::Election;
use models::Profile;
use models::SignedKey;
use models::Vote;
use models::Ballot;
use rand::thread_rng;
use sha2::Digest;
use sha2::Sha256;

// fn get_profile_obj() -> models::Profile {
//     let profile_path = Path::new("profile.json");

//     let contents = fs::read_to_string(profile_path).expect("Unable to read file");
//     return serde_json::from_str(&contents).expect("Unable to parse profile");
// }

#[tauri::command]
fn request_ballot(id: &str) -> Result<Ballot, String> {
    let ballot_path = Path::new("ballot.json");
    if ballot_path.exists() {
        return Ok(get_ballot());
    }
    let result = tauri::async_runtime::block_on(async {
        let options = Options::default();
        let rng = &mut thread_rng();
        let msg = crypto::get_keypair().pk.to_der().unwrap();
        let server_pk = http::get_server_public_key().await;

        let blinding_result = server_pk.blind(rng, &msg, false, &options).expect("Failed to blind message");

        let blind_result_data: BlindingResultData = BlindingResultData {
            blind_msg: blinding_result.blind_msg.0.clone(),
            id_number: id.trim().to_string(),
        };
        
        let sig_result: Result<SignedKey, String> = http::fetch_blind_signature(blind_result_data, blinding_result, msg, options).await;
        if sig_result.is_err() {
            return Err(sig_result.err().unwrap());
        }
        let sig = sig_result.unwrap();
        
        let ballot = Ballot{
            id: id.to_string(),
            signature: sig.sig,
        };
        let mut file = File::create(ballot_path).expect("Unable to create file");
        let ballot_json = serde_json::to_string(&ballot).expect("Failed to serialize ballot");
        file.write_all(ballot_json.as_bytes()).expect("Unable to write data");
        return Ok(ballot);
    });
    return result;
}

#[tauri::command]
fn get_ballot() -> Ballot {
    let ballot_path = Path::new("ballot.json");

    if ballot_path.exists() {
        let contents = fs::read(ballot_path).expect("Unable to read file");
        let ballot: Ballot = serde_json::from_slice(&contents).expect("Unable to parse ballot");
        return ballot;
    } else {
        return Ballot {
            id: "None".to_string(),
            signature: Signature::new(Vec::new()),
        };
    }
}

#[tauri::command]
fn submit_vote(election: &str, candidate_index: usize) -> Result<(), String> {
    let keypair = crypto::get_keypair();
    let result = tauri::async_runtime::block_on(async {
        let signature = get_ballot().signature.0.clone();

        let vote_data = format!("{}:{}", election.to_string(), candidate_index);
        let hashed_vote = Sha256::digest(vote_data.as_bytes());

        let u_sig = keypair.sk.0.clone().sign(Pkcs1v15Sign::new_raw(), &hashed_vote).expect("Failed to sign vote");

        let vote: Vote = Vote {
            election_id: election.to_string(),
            candidate_index: candidate_index,
            user_public_key: keypair.pk.to_pem().unwrap(),
            user_signature: u_sig,
            server_signature: signature,
        };
        let result = http::submit_vote(vote).await;
        return result;
    });
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    else {
        return Ok(());
    }
}

#[tauri::command]
async fn get_elections() -> Vec<Election> {
    return http::fetch_elections().await;
}

#[tauri::command]
fn create_profile(profile: Profile) {
    let profile_path = Path::new("profile.json");

    if profile_path.exists() {
        fs::remove_file(profile_path).expect("Unable to delete file");
    }

    let mut file = File::create(profile_path).expect("Unable to create file");
    let profile_json = serde_json::to_string(&profile).expect("Failed to serialize profile");
    file.write_all(profile_json.as_bytes()).expect("Unable to write data");
}

#[tauri::command]
fn get_profile() -> Result<Profile, String> {
    let profile_path = Path::new("profile.json");

    if profile_path.exists() {
        let contents = fs::read_to_string(profile_path).expect("Unable to read file");
        return Ok(serde_json::from_str(&contents).expect("Unable to parse profile"));
    } else {
        return Err("Profile not found".to_string());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![request_ballot, get_elections, create_profile, get_profile, get_ballot, submit_vote])
        .run(tauri::generate_context!())
        .expect("error whilrunning tauri application");
}
