use std::{
    fmt::{Debug, Formatter},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tauri_plugin_opener::OpenerExt;

use crate::states::AuthData;
use crate::utils::pcke_helper;

use super::{MICROSOFT_CLIENT_ID, REDIRECT_URI};

lazy_static! {
    static ref MICROSOFT_OAUTH_AUTHORIZE_URL: String =
        format!("{}/authorize", crate::account::MICROSOFT_OAUTH_API_URL);
    static ref MICROSOFT_OAUTH_TOKEN_URL: String =
        format!("{}/token", crate::account::MICROSOFT_OAUTH_API_URL);
}

#[derive(Serialize)]
pub struct OAuthTokenRequest {
    client_id: String,
    scope: String,
    code: String,
    redirect_uri: String,
    grant_type: String,
    code_verifier: String,
}

impl Debug for OAuthTokenRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OAuthTokenRequest {{ client_id: {:?}, scope: {:?}, code: [Redacted], redirect_uri: {:?}, grant_type: {:?}, code_verifier: [Redacted] }}", self.client_id, self.scope, self.redirect_uri, self.grant_type)
    }
}

#[derive(Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub ext_expires_in: u32,
    pub scope: String,
    pub refresh_token: String,
}

impl Debug for OAuthTokenResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OAuthTokenResponse {{ access_token: [Redacted], token_type: {:?}, expires_in: {:?}, ext_expires_in: {:?}, scope: {:?}, refresh_token: [Redacted] }}", self.token_type, self.expires_in, self.ext_expires_in, self.scope)
    }
}

pub async fn request_access_token(
    code: &str,
    code_verifier: &str,
) -> Result<OAuthTokenResponse, anyhow::Error> {
    let client = reqwest::Client::new();

    let form = OAuthTokenRequest {
        client_id: MICROSOFT_CLIENT_ID.to_string(),
        scope: "XboxLive.signin".to_string(),
        code: code.to_string(),
        redirect_uri: REDIRECT_URI.to_string(),
        grant_type: "authorization_code".to_string(),
        code_verifier: code_verifier.to_string(),
    };

    log::debug!(
        "Sending request to Microsoft OAuth Token URL with params: {:?}",
        form
    );

    let response = client
        .post(MICROSOFT_OAUTH_TOKEN_URL.clone())
        .form(&form)
        .send()
        .await?;

    if response.error_for_status_ref().is_err() {
        log::error!(
            "Unable to retrieve access token: {:?}",
            response.text().await?
        );
        return Err(anyhow::anyhow!("Unable to retrieve access token"));
    }

    let content: OAuthTokenResponse = match response.json().await {
        Ok(content) => content,
        Err(e) => {
            log::error!("Unable to parse response: {:?}", e);
            return Err(anyhow::anyhow!("Unable to parse response"));
        }
    };

    Ok(content)
}

#[tauri::command]
pub fn open_microsoft_oauth(
    app_handle: tauri::AppHandle,
    auth_data: tauri::State<Mutex<AuthData>>,
) {
    let scope = "XboxLive.signin+Xboxlive.offline_access";

    let state = rand::random::<[u8; 32]>()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let mut auth_data = auth_data
        .lock()
        .expect("mutex already locked by the current thread");
    auth_data.state = Some(state.clone());

    let pcke = pcke_helper::generate_pcke_challenge();
    auth_data.code_verifier = Some(pcke.code_verifier);

    let url = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        *MICROSOFT_OAUTH_AUTHORIZE_URL, MICROSOFT_CLIENT_ID, REDIRECT_URI, scope, state, pcke.code_challenge
    );

    log::debug!("Opening URL in browser: {}", url);

    if let Err(e) = app_handle.opener().open_url(url, None::<&str>) {
        log::error!("failed to open URL in default browser: {}", e);
    }
}
