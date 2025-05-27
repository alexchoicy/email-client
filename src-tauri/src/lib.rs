use std::sync::Mutex;

use database::OAuthDatabaseManager::OAuthDatabaseManger;
use tauri::Manager;
mod database;
mod email_provider;
mod oauth;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let oauth_db_manager = OAuthDatabaseManger::new(&app_handle);
            app.manage(oauth_db_manager);

            let config_manager = utils::config::Config::new(&app_handle);
            app.manage(Mutex::new(config_manager));

            let http_client = utils::http_client::create_http_client();
            app.manage(http_client);
            Ok(())
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            oauth::google_auth::start_google_oauth,
            utils::config::get_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
