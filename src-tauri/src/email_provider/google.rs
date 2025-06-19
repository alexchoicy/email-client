use std::{collections::HashMap, sync::Mutex};
use tokio::time::{sleep, Duration};

use super::google_types::{GoogleEmailList, GoogleProfile};
use crate::{email_provider::google_types::GoogleEmailListMail, stateTokenData};
use std::fmt::Write;
use tauri::http::{HeaderMap, HeaderValue};
use tauri_plugin_http::reqwest;

pub async fn get_google_user_profile(
    token: &String,
    http_client: &tauri::State<'_, reqwest::Client>,
) -> Result<GoogleProfile, String> {
    let url = "https://www.googleapis.com/oauth2/v3/userinfo".to_string();

    let response = http_client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if response.status().is_success() {
        let profile: GoogleProfile = response
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

// new account -> get messages.list -> loop until no "nextPageToken" -> use magical Batch Retrieve with messages.get
// to get all messages --> save the HistoryID

// sync -> use history.list with the saved HistoryID -> loop until no "nextPageToken"
// -> if has any "messagesAdded"/"messagesDeleted"/"labelsAdded"/"labelsRemoved" ->
// use magical Batch Retrieve with messages.get to get all messages, update the historyID from history.list

pub async fn get_google_email_list(
    http_client: &tauri::State<'_, reqwest::Client>,
    access_token: &String,
) -> (Result<Vec<GoogleEmailListMail>, bool>) {
    const BASE_URL: &str = "https://gmail.googleapis.com/gmail/v1/users/me/messages";
    let mut email_lists: Vec<GoogleEmailListMail> = Vec::new();
    let mut isLastPage = false;

    let mut pageToken: Option<String> = None;
    while (!isLastPage) {
        let url = if let Some(ref page_token) = pageToken {
            format!("{}?pageToken={}", BASE_URL, page_token)
        } else {
            BASE_URL.to_string()
        };

        let response = http_client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .expect("Failed to send request");

        if response.status().is_success() {
            let email_list: GoogleEmailList = response
                .json()
                .await
                .expect("Failed to parse JSON response");
            let messages_len = email_list.messages.len();
            email_lists.extend(email_list.messages);

            println!(
                "Received email list with {} messages, nextPageToken: {:?}, verified: {:?}",
                messages_len, email_list.nextPageToken, email_list.resultSizeEstimate
            );

            if (email_list.nextPageToken.is_some()) {
                pageToken = email_list.nextPageToken;
            } else {
                isLastPage = true;
            }
        } else {
            return (Err(false));
        }
    }
    return Ok(email_lists);
}

pub async fn get_google_email_content_with_branch(
    http_client: &tauri::State<'_, reqwest::Client>,
    access_token: &String,
    email_list: Vec<GoogleEmailListMail>,
) {
    let mut body = String::new();
    const BOUNDARY: &str = "BATCH_BOUNDARY_STRING";

    for email_id in email_list {
        writeln!(body, "--{}", BOUNDARY).unwrap();
        writeln!(body, "Content-Type: application/http").unwrap();
        writeln!(body, "Content-ID: <{}>", email_id.id).unwrap();
        writeln!(body, "").unwrap();
        writeln!(body, "GET /gmail/v1/users/me/messages/{}", email_id.id).unwrap();
        writeln!(body, "Host: www.googleapis.com").unwrap();
        writeln!(body, "").unwrap();
    }

    writeln!(body, "--{}--", BOUNDARY).unwrap();

    let mut header = HeaderMap::new();
    header.insert(
        "Content-Type",
        HeaderValue::from_str(&format!("multipart/mixed; boundary={}", BOUNDARY)).unwrap(),
    );
    header.insert(
        "AUTHORIZATION",
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );
    header.insert(
        "Content-Length",
        HeaderValue::from_str(&body.len().to_string()).unwrap(),
    );

    let response = http_client
        .post("https://gmail.googleapis.com/batch/gmail/v1")
        .headers(header)
        .body(body)
        .send()
        .await
        .expect("Failed to send batch request");

    if !response.status().is_success() {
        println!("Failed to fetch email content: {}", response.status());
        return;
    }

    let response_text = response.text().await.expect("Failed to read response text");

    let temp = http_client
        .post("http://localhost:3000/")
        .body(response_text)
        .send()
        .await
        .expect("Failed to send email content to local server");
}

pub async fn init_new_google_account(
    http_client: &tauri::State<'_, reqwest::Client>,
    access_token: &String,
) {
    const CALL_RATE_LIMIT: usize = 20;

    let email_lists = get_google_email_list(http_client, access_token)
        .await
        .expect("Failed to get email list");

    println!("Received {} email lists", email_lists.len());

    let total_chunks = email_lists.chunks(CALL_RATE_LIMIT).count();
    let mut chunk_count = 0;

    for chunk in email_lists.chunks(CALL_RATE_LIMIT) {
        chunk_count += 1;
        println!("Processing chunk {} of {}", chunk_count, total_chunks);

        get_google_email_content_with_branch(http_client, access_token, chunk.to_vec()).await;

        if chunk_count < total_chunks {
            println!("Sleeping for 500ms before next chunk...");
            sleep(Duration::from_millis(500)).await;
        }
    }
    println!("Finished processing all email content chunks.");
}
