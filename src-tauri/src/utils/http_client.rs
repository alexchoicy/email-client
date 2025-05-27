use tauri_plugin_http::reqwest;

pub fn create_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("Tauri App HTTP Client")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client")
}
