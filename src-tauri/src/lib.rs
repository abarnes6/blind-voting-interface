// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    "yo".to_string()
}

#[tauri::command]
fn generate_identity() -> bool {
    true
}

#[tauri::command]
fn request_ballot() -> bool {
    true
}

#[tauri::command]
fn submit_ballot(name: &str) -> bool {
    true
}

#[tauri::command]
fn get_elections() -> Vec<String> {
    vec!["election1".to_string(), "election2".to_string()]
}

#[tauri::command]
fn get_candidates(election: &str) -> Vec<String> {
    vec!["candidate1".to_string(), "candidate2".to_string()]
}

#[tauri::command]



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, generate_identity, request_ballot, submit_ballot])
        .run(tauri::generate_context!())
        .expect("error whilrunning tauri application");
}
