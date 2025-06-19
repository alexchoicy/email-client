use std::{collections::HashMap, sync::Mutex};

use database::OAuthDatabaseManager::OAuthDatabaseManger;
use oauth::auth_callback_server::OAuthEmailProvider;
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

pub struct stateTokenData {
    pub identifier: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub email_provider: OAuthEmailProvider,
}

pub async fn loadToken(app: &tauri::AppHandle, oauthDatabaseManger: &OAuthDatabaseManger) {
    let token_from_db = oauthDatabaseManger
        .get_toke_data()
        .await
        .expect("Failed to get token data from database");

    let mut tokens = HashMap::<String, stateTokenData>::new();

    for token_data in token_from_db {
        tokens.insert(
            token_data.identifier.clone(), // Use identifier as key
            stateTokenData {
                identifier: token_data.identifier,
                access_token: token_data.access_token,
                refresh_token: token_data.refresh_token,
                expires_at: token_data.expires_at,
                email_provider: token_data.email_provider,
            },
        );
    }
    app.manage(Mutex::new(tokens)); // Manage the HashMap, wrapped in Mutex
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            let oauth_db_manager = OAuthDatabaseManger::new(&app_handle);
            app.manage(oauth_db_manager);

            let config_manager = utils::config::Config::new(&app_handle);
            app.manage(Mutex::new(config_manager));

            let http_client = utils::http_client::create_http_client();
            app.manage(http_client);

            let app_handle_for_task = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                let oauth_db_state: tauri::State<'_, OAuthDatabaseManger> =
                    app_handle_for_task.state::<OAuthDatabaseManger>();
                loadToken(&app_handle_for_task, &oauth_db_state).await;
            });

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
