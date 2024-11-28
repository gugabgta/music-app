use std::env;

use tauri::{path::BaseDirectory, Manager};

#[tauri::command]
fn get_env(key: &str) -> String {
    dotenv::var(key).unwrap_or_else(|_| format!("{key} not set").to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_env])
        .setup(|app| {
            let resource_path = app.path().resolve("icons/icon.json", BaseDirectory::Resource)?;
            dbg!(resource_path);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
