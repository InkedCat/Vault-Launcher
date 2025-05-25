use std::sync::Mutex;

use lazy_static::lazy_static;
use tauri_plugin_opener::OpenerExt;

use crate::states::AuthData;
use crate::utils::pcke_helper;

use super::{MICROSOFT_CLIENT_ID, REDIRECT_URI};

lazy_static! {
    static ref MICROSOFT_OAUTH_AUTHORIZE_URL: String =
        format!("{}/authorize", crate::account::MICROSOFT_OAUTH_API_URL);
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

    let mut auth_data = auth_data.lock().unwrap();
    auth_data.state = Some(state.clone());

    let pcke = pcke_helper::generate_pcke_challenge();
    auth_data.code_verifier = Some(pcke.code_verifier.clone());

    let url = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        *MICROSOFT_OAUTH_AUTHORIZE_URL, MICROSOFT_CLIENT_ID, REDIRECT_URI, scope, state, pcke.code_challenge
    );

    log::debug!("Opening URL in browser: {}", url);

    app_handle.opener().open_url(url, None::<&str>).unwrap();
}
