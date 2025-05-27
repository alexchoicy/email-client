use std::sync::Mutex;

use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    EndpointNotSet, EndpointSet, PkceCodeChallenge, RedirectUrl, RevocationErrorResponseType,
    Scope, StandardErrorResponse, StandardRevocableToken, StandardTokenIntrospectionResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
};

use rusqlite::config;
use tauri::{AppHandle, Runtime};
use tauri_plugin_http::reqwest;
use tauri_plugin_opener::OpenerExt;
use url::Url;

use crate::database::OAuthDatabaseManager::{OAuthDatabaseManger, OAuthTokenData};
use crate::oauth::auth_callback_server::temp_oauth_callback_server;

use super::auth_callback_server::{
    get_available_ports, get_oauth_redirect_url, OAuthEmailProvider,
};

type Client = oauth2::Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Debug)]
struct GoogleOAuthParams {
    state: String,
    code: String,
    scope: String,
    authuser: String,
    prompt: String,
}

fn create_google_oauth_client(redirect_url: &str) -> Result<Client, String> {
    let client_id = dotenvy_macro::dotenv!("GOOGLE_CLIENT_ID").to_string();
    let auth_url = dotenvy_macro::dotenv!("GOOGLE_AUTH_URL").to_string();
    let token_url = dotenvy_macro::dotenv!("GOOGLE_TOKEN_URL").to_string();
    let client_secret = dotenvy_macro::dotenv!("GOOGLE_CLIENT_SECRET").to_string();

    let client = BasicClient::new(ClientId::new(client_id))
        .set_auth_uri(AuthUrl::new(auth_url).map_err(|_| "Invalid GOOGLE_AUTH_URL".to_string())?)
        .set_token_uri(
            TokenUrl::new(token_url).map_err(|_| "Invalid GOOGLE_TOKEN_URL".to_string())?,
        )
        .set_client_secret(ClientSecret::new(client_secret))
        .set_redirect_uri(
            RedirectUrl::new(redirect_url.to_string())
                .map_err(|_| "Invalid redirect URL".to_string())?,
        );

    Ok(client)
}

fn extract_google_oauth_params(
    params: &std::collections::HashMap<String, String>,
) -> Result<GoogleOAuthParams, String> {
    Ok(GoogleOAuthParams {
        state: params
            .get("state")
            .ok_or_else(|| "Missing 'state' in OAuth params".to_string())?
            .clone(),
        code: params
            .get("code")
            .ok_or_else(|| "Missing 'code' in OAuth params".to_string())?
            .clone(),
        scope: params
            .get("scope")
            .ok_or_else(|| "Missing 'scope' in OAuth params".to_string())?
            .clone(),
        authuser: params
            .get("authuser")
            .ok_or_else(|| "Missing 'authuser' in OAuth params".to_string())?
            .clone(),
        prompt: params
            .get("prompt")
            .ok_or_else(|| "Missing 'prompt' in OAuth params".to_string())?
            .clone(),
    })
}

fn build_authorize_url(client: &Client, pkce_challenge: PkceCodeChallenge) -> (Url, CsrfToken) {
    client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://mail.google.com/".to_string()))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url()
}

type TokenResponseContent = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

async fn save_google_oauth(
    token_response: TokenResponseContent,
    oauth_db: &OAuthDatabaseManger,
) -> Result<(), String> {
    let data = OAuthTokenData {
        identifier: token_response.access_token().secret().to_string(),
        email: "test".to_string(),
        access_token: token_response.access_token().secret().to_string(),
        refresh_token: token_response
            .refresh_token()
            .map(|rt| rt.secret().to_string())
            .unwrap_or_default(),
        expires_at: token_response
            .expires_in()
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0),
        token_type: "Bearer".to_string(),
        email_provider: OAuthEmailProvider::Google,
    };

    // If save_token_data is async, use .await, otherwise call directly
    oauth_db
        .save_token_data(data)
        .await
        .map(|_| {
            println!("Google OAuth token data saved successfully.");
        })
        .map_err(|e| {
            eprintln!("Failed to save Google OAuth token data: {}", e);
            e.to_string()
        })
}

#[tauri::command]
pub async fn start_google_oauth(
    app: tauri::AppHandle,
    oauth_db: tauri::State<'_, OAuthDatabaseManger>,
    http_client: tauri::State<'_, reqwest::Client>,
    config: tauri::State<'_, Mutex<crate::utils::config::Config>>,
) -> Result<String, bool> {
    println!("Starting Google OAuth flow...");
    let port = get_available_ports();
    let redirect_url = get_oauth_redirect_url(port, OAuthEmailProvider::Google);

    let client = create_google_oauth_client(&redirect_url).map_err(|e| {
        eprintln!("Failed to create Google OAuth client: {}", e);
        false
    })?;
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, _csrf_token) = build_authorize_url(&client, pkce_challenge);

    let _ = app.opener().open_path(authorize_url, None::<&str>);

    let params =
        temp_oauth_callback_server(port, OAuthEmailProvider::Google).map_err(|_e| false)?;

    let google_params = extract_google_oauth_params(&params).map_err(|e| {
        eprintln!("Failed to extract Google OAuth params: {}", e);
        false
    })?;

    let token_response_result = client
        .exchange_code(AuthorizationCode::new(google_params.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&*http_client)
        .await;

    println!("Received OAuth params: {:#?}", token_response_result);

    match token_response_result {
        Ok(response) => {
            let profile = crate::email_provider::google::get_google_user_profile(
                &response.access_token().secret(),
                http_client,
            )
            .await;
            println!("Google User Profile: {:#?}", profile);
            let _ = save_google_oauth(response.clone(), &oauth_db).await;
            let mut config = config.lock().unwrap();
            config.set_has_account(&app);

            Ok(serde_json::to_string(&profile)
                .unwrap_or_else(|_| "Failed to serialize token response".to_string()))
        }
        Err(e) => {
            eprintln!("Error during token exchange: {:#?}", e);
            if let oauth2::RequestTokenError::ServerResponse(server_error) = &e {
                eprintln!("OAuth Server Error Details:");
                eprintln!("  Error Type: {:?}", server_error.error());
                if let Some(desc) = server_error.error_description() {
                    eprintln!("  Description: {}", desc);
                }
                if let Some(uri) = server_error.error_uri() {
                    eprintln!("  More Info URI: {}", uri.as_str());
                }
            } else {
                eprintln!("An unclassified OAuth error occurred during token exchange.");
            }
            Err(false)
        }
    }
}
