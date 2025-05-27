use tauri_plugin_http::reqwest;

pub async fn get_google_user_profile(
    token: &String,
    http_client: tauri::State<'_, reqwest::Client>,
) -> Result<serde_json::Value, String> {
    let url = "https://www.googleapis.com/oauth2/v3/userinfo".to_string();

    let response = http_client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if response.status().is_success() {
        let profile: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON response: {}", e))?;
        Ok(profile)
    } else {
        Err(format!(
            "Failed to fetch user profile: {}",
            response.status()
        ))
    }
}
