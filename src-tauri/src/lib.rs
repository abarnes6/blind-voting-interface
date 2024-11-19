// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn generate_identity() -> bool {
    true
}

#[tauri::command]
fn request_ballot() -> bool {
    true
}

#[tauri::command]
fn submit_ballot(election: &str, name: &str) -> bool {
    if election == "election1" {
        if name == "candidate1" {
            return true
        }
        else if name == "candidate2" {
            return true
        }
    }
    else if election == "election2" {
        if name == "candidate3" {
            return true
        }
        else if name == "candidate4" {
            return true
        }
    }
    return false
}

#[tauri::command]
fn get_elections() -> Vec<String> {
    vec!["election1".to_string(), "election2".to_string()]
}

#[tauri::command]
fn get_candidates(election: &str) -> Vec<String> {
    if election == "election1" {
        return vec!["candidate1".to_string(), "candidate2".to_string()]
    }
    else if election == "election2" {
        return vec!["candidate3".to_string(), "candidate4".to_string()]
    }
    else {
        return vec![]
    }
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![generate_identity, request_ballot, submit_ballot, get_elections, get_candidates])
        .run(tauri::generate_context!())
        .expect("error whilrunning tauri application");
}
