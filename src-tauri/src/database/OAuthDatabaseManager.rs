use crate::oauth::auth_callback_server::OAuthEmailProvider;
use rusqlite::Connection;
use std::fs;
use std::sync::Mutex;
use tauri::Manager;

pub struct OAuthTokenData {
    pub identifier: String,
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub token_type: String,
    pub email_provider: OAuthEmailProvider,
}

pub struct OAuthDatabaseManger {
    db: Mutex<Connection>,
}

impl OAuthDatabaseManger {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data directory");

        if !app_data_dir.exists() {
            fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
        }

        let db_path = app_data_dir.join("oauth_tokens.db");

        let db = Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database at {:?}: {}", db_path, e))
            .expect("Failed to open database");

        db.execute(
            "CREATE TABLE IF NOT EXISTS oauth_tokens (
            IDENTIFIER TEXT PRIMARY KEY,
            email TEXT,
            access_token TEXT NOT NULL,
            refresh_token TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            token_type TEXT NOT NULL,
            email_provider TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create oauth_tokens table");
        Self { db: Mutex::new(db) }
    }

    pub async fn save_token_data(&self, data: OAuthTokenData) -> Result<(), String> {
        let conn = self.db.lock().map_err(|e| e.to_string())?;
        conn
            .execute(
                "INSERT OR REPLACE INTO oauth_tokens (IDENTIFIER, email, access_token, refresh_token, expires_at, token_type, email_provider) VALUES (?, ?, ?, ?, ?, ?, ?)",
                rusqlite::params![
                    data.identifier,
                    data.email,
                    data.access_token,
                    data.refresh_token,
                    data.expires_at,
                    data.token_type,
                    data.email_provider.as_str(),
                ],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
