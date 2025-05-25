use std::net::TcpListener;
use tiny_http::{Header, Method, Response, Server, StatusCode};
use url::Url;

const OAUTH_SERVER_IP: &str = "127.0.0.1";

pub enum OAuthEmailProvider {
    Google,
    Microsoft,
}

impl OAuthEmailProvider {
    pub fn as_str(&self) -> &str {
        match self {
            OAuthEmailProvider::Google => "google",
            OAuthEmailProvider::Microsoft => "microsoft",
        }
    }
}

pub fn get_available_ports() -> u16 {
    (8000..9000)
        .find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
        .expect("No available ports found in range 8000..9000")
}

pub fn get_oauth_redirect_url(port: u16, oauth_email_provider: OAuthEmailProvider) -> String {
    let base_url = format!("http://{}:{}", OAUTH_SERVER_IP, port);
    let provider_str = oauth_email_provider.as_str();
    format!("{}/oauth/callback/{}", base_url, provider_str)
}

pub fn temp_oauth_callback_server(
    port: u16,
    oauth_email_provider: OAuthEmailProvider,
) -> Result<std::collections::HashMap<String, String>, String> {
    let address = format!("{}:{}", OAUTH_SERVER_IP, port);
    let redirect_url = format!("/oauth/callback/{}", oauth_email_provider.as_str());

    // let server = Server::http(&address).map_err(|e| format!("Server error: {}", e));
    let server = Server::http(&address).map_err(|e| format!("Server error: {}", e))?;

    match server.recv_timeout(std::time::Duration::from_secs(300)) {
        Ok(Some(request)) => {
            if request.method() == &Method::Get && request.url().starts_with(&redirect_url) {
                if let Ok(url) = Url::parse(&format!("http://localhost{}{}", request.url(), "")) {
                    let params: std::collections::HashMap<_, _> =
                        url.query_pairs().into_owned().collect();
                    println!("Received OAuth callback with params: {:?}", params);
                    if let Some(_code) = params.get("code") {
                        let response = Response::from_string(
                            "<h1>Authentication successful! You can close this window.</h1>",
                        )
                        .with_header(
                            Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap(),
                        )
                        .with_status_code(StatusCode(200));
                        let _ = request.respond(response);
                        return Ok(params);
                    } else {
                        let response = Response::from_string("<h1>Invalid request</h1>")
                            .with_status_code(StatusCode(400));
                        let _ = request.respond(response);
                        return Err("Invalid OAuth callback request".into());
                    }
                }
            }
            return Err("Invalid request".into());
        }

        Ok(None) => {
            eprintln!("No request received within the timeout period.");
            return Err("No request received within the timeout period.".into());
        }

        Err(e) => {
            eprintln!("Error receiving request: {}", e);
            return Err(format!("Error receiving request: {}", e));
        }
    }
}
