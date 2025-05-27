use std::sync::Mutex;

use serde::Deserialize;
use tauri::Manager;
use thiserror::Error;

use crate::states::AuthData;

use super::{microsoft, xbox};

#[derive(Error, Debug)]
pub enum LoginCallbackError {
    #[error("Unable to parse query: {0}")]
    ParseQueryError(#[from] serde_qs::Error),
    #[error("State mismatch: {0} != {1}")]
    StateMismatch(String, String),
    #[error("No state found, cannot verify login")]
    NoStateFound,
    #[error("No code verifier found, cannot verify login")]
    NoCodeVerifierFound,
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

    let oauth_token_response = microsoft::request_access_token(&params.code, &code_verifier).await;

    if let Ok(oauth_token_response) = oauth_token_response {
        let xbox_live_token = xbox::login_to_xbox_live(&oauth_token_response.access_token).await;
        log::info!("Xbox Live Token: {:?}", xbox_live_token);
    } else {
        log::error!(
            "Unable to retrieve access token: {:?}",
            oauth_token_response.err().as_ref()
        );
    }

    Ok(())
}
