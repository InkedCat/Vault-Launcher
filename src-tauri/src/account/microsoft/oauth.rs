use std::fmt::Write;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri_plugin_opener::OpenerExt;
use thiserror::Error;

use crate::{states::AuthData, utils::pcke_helper, REQWEST_CLIENT};

use super::{
    MICROSOFT_CLIENT_ID, MICROSOFT_OAUTH_AUTHORIZE_URL, MICROSOFT_OAUTH_TOKEN_URL, REDIRECT_URI,
};

const SCOPE: &str = "XboxLive.signin Xboxlive.offline_access";

#[tauri::command]
pub fn open_microsoft_login(
    app_handle: tauri::AppHandle,
    auth_data: tauri::State<Mutex<AuthData>>,
) {
    let state = rand::random::<[u8; 32]>()
        .iter()
        .fold(String::new(), |mut acc, b| {
            write!(&mut acc, "{:02x}", b).unwrap();
            acc
        });

    let mut auth_data = auth_data
        .lock()
        .expect("mutex already locked by the current thread");
    auth_data.state = Some(state.clone());

    let pcke = pcke_helper::generate_pcke_challenge();
    auth_data.code_verifier = Some(pcke.code_verifier);

    let url = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        *MICROSOFT_OAUTH_AUTHORIZE_URL, MICROSOFT_CLIENT_ID, REDIRECT_URI, SCOPE, state, pcke.code_challenge
    );

    log::debug!("Opening URL in browser: {}", url);

    if let Err(e) = app_handle.opener().open_url(url, None::<&str>) {
        log::error!("failed to open URL in default browser: {}", e);
    }
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

#[derive(Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub ext_expires_in: u32,
    pub scope: String,
    pub refresh_token: String,
}

#[derive(Error, Debug)]
pub enum OAuthTokenError {
    #[error("HTTP request error: {0}")]
    HTTPRequestError(reqwest::Error),
    #[error("OAuth flow error: {0}")]
    OAuthFlowError(reqwest::Error),
    #[error("Failed to parse response: {0}")]
    InvalidResponseFormat(String),
}

pub async fn request_access_token(
    code: &str,
    code_verifier: &str,
) -> Result<OAuthTokenResponse, OAuthTokenError> {
    let form = OAuthTokenRequest {
        client_id: MICROSOFT_CLIENT_ID.to_string(),
        scope: SCOPE.to_string(),
        code: code.to_string(),
        redirect_uri: REDIRECT_URI.to_string(),
        grant_type: "authorization_code".to_string(),
        code_verifier: code_verifier.to_string(),
    };

    log::debug!("Sending request to Microsoft OAuth Token URL");

    let response = match REQWEST_CLIENT
        .post(MICROSOFT_OAUTH_TOKEN_URL.clone())
        .form(&form)
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return Err(OAuthTokenError::HTTPRequestError(error));
        }
    };

    if let Err(error) = response.error_for_status_ref() {
        return Err(OAuthTokenError::OAuthFlowError(error));
    }

    let content = match response.json().await {
        Ok(content) => content,
        Err(error) => {
            return Err(OAuthTokenError::InvalidResponseFormat(error.to_string()));
        }
    };

    Ok(content)
}
