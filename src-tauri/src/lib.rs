mod models;
mod crypto;

use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use blind_rsa_signatures::Signature;
use models::Profile;
use serde_json::json;

fn get_profile_obj() -> models::Profile {
    let profile_path = Path::new("profile.json");

    let contents = fs::read_to_string(profile_path).expect("Unable to read file");
    return serde_json::from_str(&contents).expect("Unable to parse profile");
}

#[tauri::command]
fn request_ballot() -> bool {
    let ballot_path = Path::new("ballot.json");
    if ballot_path.exists() {
        return false;
    }
    let profile = get_profile_obj();
    tauri::async_runtime::block_on(async {
        let sig: Signature = crypto::request_ballot(profile.clone(), crypto::get_keypair()).await;
        let mut file = File::create(ballot_path).expect("Unable to create file");

        let ballot_profile = Profile {
            first_name: profile.first_name,
            last_name: profile.last_name,
            address: profile.address,
            dob: profile.dob,
            driv_lic: profile.driv_lic,
            public_key: crypto::get_keypair().pk,
            signature: Some(sig),
        };

        file.write_all(serde_json::to_string(&ballot_profile).expect("Could not convert to string.").as_bytes()).expect("Unable to write data");
    });

    return true;
}

#[tauri::command]
fn get_ballot() -> String {
    let ballot_path = Path::new("ballot.json");

    if ballot_path.exists() {
        let contents = fs::read_to_string(ballot_path).expect("Unable to read file");
        return contents;
    } else {
        return "None".to_string();
    }
}

#[tauri::command]
fn submit_ballot(election: &str, candidate: &str) -> bool {
    let keypair = crypto::get_keypair();

    let ballot_path = Path::new("ballot.json");
    let ballot_contents = fs::read_to_string(ballot_path).expect("Unable to read file");
    let ballot: models::Profile = serde_json::from_str(&ballot_contents).expect("Unable to parse ballot");
    crypto::submit_ballot(election, candidate, keypair, ballot.signature.unwrap());

    return true;
}

#[tauri::command]
async fn get_elections() -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:7878/get-elections")
        .send()
        .await
        .expect("Failed to send request");
    let elections: Vec<String> = response.json().await.map_err(|_| "Failed to parse elections".to_string())?;
    Ok(elections)
}

#[tauri::command]
async fn get_candidates(election: &str) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:7878/get-candidates?election=".to_string() + election)
        .send()
        .await
        .expect("Failed to send request");
    let candidates: Vec<String> = response.json().await.map_err(|_| "Failed to parse candidates".to_string())?;
    Ok(candidates)
}

#[tauri::command]
fn create_profile(first_name: &str, last_name: &str, address: &str, dob: &str, driv_lic: &str) -> bool {
    let profile_path = Path::new("profile.json");

    if profile_path.exists() {
        return false;
    } else {
        let profile = json!({
            "first_name": first_name,
            "last_name": last_name,
            "address": address,
            "dob": dob,
            "driv_lic": driv_lic
        });

        let mut file = File::create(profile_path).expect("Unable to create file");
        file.write_all(profile.to_string().as_bytes()).expect("Unable to write data");
        return true;
    }
}

#[tauri::command]
fn get_profile() -> String {
    let profile_path = Path::new("profile.json");

    if profile_path.exists() {
        let contents = fs::read_to_string(profile_path).expect("Unable to read file");
        return contents;
    } else {
        return "None".to_string();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![request_ballot, submit_ballot, get_elections, get_candidates, create_profile, get_profile])
        .run(tauri::generate_context!())
        .expect("error whilrunning tauri application");
}
