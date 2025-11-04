// Import our database module
mod db;

use db::test_db_connection;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Tauri command to test database connectivity
/// This will be called from the frontend via invoke("test_database")
#[tauri::command]
fn test_database() -> Result<String, String> {
    // Call our test function and map errors to strings
    test_db_connection().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, test_database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
