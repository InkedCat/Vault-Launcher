pub mod oauth;
pub mod xbox_live;

use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::Deserialize;
use tauri::Manager;
use thiserror::Error;

use crate::states::AuthData;

const MICROSOFT_OAUTH_API_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0";
const XBOX_LIVE_AUTH_DOMAIN: &str = "auth.xboxlive.com";

const MICROSOFT_CLIENT_ID: &str = env!("MICROSOFT_CLIENT_ID", "MICROSOFT_CLIENT_ID is not defined");
const REDIRECT_URI: &str = "vault-launcher://account/login/callback";

lazy_static! {
    static ref MICROSOFT_OAUTH_AUTHORIZE_URL: String =
        format!("{}/authorize", MICROSOFT_OAUTH_API_URL);
    static ref MICROSOFT_OAUTH_TOKEN_URL: String = format!("{}/token", MICROSOFT_OAUTH_API_URL);
}

#[derive(Error, Debug)]
pub enum LoginCallbackError {
    #[error("Failed to parse query: {0}")]
    InvalidQueryFormat(#[from] serde_qs::Error),
    #[error("State mismatch: {0} != {1}")]
    StateMismatch(String, String),
    #[error("No state found, cannot verify login")]
    NoStateFound,
    #[error("No code verifier found, cannot verify login")]
    NoCodeVerifierFound,
    #[error("Failed to request access token: {0}")]
    TokenRequestError(oauth::OAuthTokenError),
    #[error("Failed to login to Xbox Live: {0}")]
    XboxLiveLoginError(xbox_live::XboxLiveError),
}

#[derive(Deserialize, Debug)]
struct LoginCallback {
    state: String,
    code: String,
}

pub async fn handle_callback(
    app_handle: &tauri::AppHandle,
    query: &str,
) -> Result<(), LoginCallbackError> {
    let params = serde_qs::from_str::<LoginCallback>(query)?;

    let auth_mutex = app_handle.state::<Mutex<AuthData>>();
    let (state, code_verifier) = {
        let auth_data = auth_mutex.lock().expect("lock is poisoned");

        let state = auth_data.state.clone().ok_or_else(|| {
            log::error!("No state found, cannot verify login");
            LoginCallbackError::NoStateFound
        })?;

        let code_verifier = auth_data.code_verifier.clone().ok_or_else(|| {
            log::error!("No code verifier found, cannot verify login");
            LoginCallbackError::NoCodeVerifierFound
        })?;

        (state, code_verifier)
    };

    if state != params.state {
        log::error!("State mismatch: {} != {}", state, params.state);
        return Err(LoginCallbackError::StateMismatch(
            state.to_string(),
            params.state.to_string(),
        ));
    }

    let oauth_response = match oauth::request_access_token(&params.code, &code_verifier).await {
        Ok(response) => response,
        Err(error) => {
            log::error!("Failed to request access token: {:?}", error);
            return Err(LoginCallbackError::TokenRequestError(error));
        }
    };

    let xbox_live_response = match xbox_live::login_to_xbox_live(&oauth_response.access_token).await
    {
        Ok(response) => response,
        Err(error) => {
            log::error!("Failed to login to Xbox Live: {:?}", error);
            return Err(LoginCallbackError::XboxLiveLoginError(error));
        }
    };

    let xsts_response = match xbox_live::get_xsts_token(&xbox_live_response.token).await {
        Ok(response) => response,
        Err(error) => {
            log::error!("Failed to get XSTS token: {:?}", error);
            return Err(LoginCallbackError::XboxLiveLoginError(error));
        }
    };

    let xbox_live_uhs = xbox_live_response
        .display_claims
        .xui
        .first()
        .expect("No XUI found")
        .uhs
        .clone();
    let xsts_uhs = xsts_response
        .display_claims
        .xui
        .first()
        .expect("No XUI found")
        .uhs
        .clone();

    if xbox_live_uhs != xsts_uhs {
        log::error!("XSTS uhs mismatch: {:?}", xsts_response.token);
    }

    println!("XSTS response: {:?}", xsts_response.token);

    Ok(())
}
