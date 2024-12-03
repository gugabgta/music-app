use std::env;

use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_http::reqwest;

#[tauri::command]
fn get_env(key: &str) -> String {
    dotenv::var(key).unwrap_or_else(|_| format!("{key} not set").to_string())
}

#[tauri::command]
async fn get_playlist() -> String {
    let url = dotenv::var("API_URL").unwrap_or("http://127.0.0.1:3000".to_string());

    println!("url: {}", url);

    let playlist: Vec<serde_json::Value> = reqwest::get(format!("{}/playlist", url))
        .await
        .expect("")
        .json()
        .await
        .unwrap();

    serde_json::to_string(&playlist).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_env, 
            get_playlist,
        ])
        .setup(|app| {
            let resource_path = app
                .path()
                .resolve("icons/icon.json", BaseDirectory::Resource)?;
            dbg!(resource_path);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
