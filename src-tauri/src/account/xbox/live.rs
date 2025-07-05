use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::http::HeaderMap;
use thiserror::Error;

use crate::REQWEST_CLIENT;

use super::XBOX_LIVE_AUTH_URL;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XboxLiveTokenRequest<T> {
    properties: T,
    relying_party: String,
    token_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct LiveRequestProperties {
    auth_method: String,
    site_name: String,
    rps_ticket: String,
}

#[derive(Deserialize)]
pub struct Xui {
    pub uhs: String,
}

#[derive(Deserialize)]
pub struct DisplayClaims {
    pub xui: Vec<Xui>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveResponse {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: DisplayClaims,
}

#[derive(Error, Debug)]
pub enum XboxLiveError {
    #[error("HTTP request error: {0}")]
    HTTPRequestError(reqwest::Error),
    #[error("Xbox Live login error: {0}")]
    XboxLiveLoginError(reqwest::Error),
    #[error("Failed to parse response: {0}")]
    InvalidResponseFormat(String),
}

pub async fn login_to_xbox_live(token: &str) -> Result<XboxLiveResponse, XboxLiveError> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/json"
            .parse()
            .expect("Failed to parse content type header"),
    );
    headers.insert(
        "Accept",
        "application/json"
            .parse()
            .expect("Failed to parse accept header"),
    );
    headers.insert(
        "x-xbl-contract-version",
        "1".parse()
            .expect("Failed to parse contract version header"),
    );

    let body = json!(XboxLiveTokenRequest {
        properties: LiveRequestProperties {
            auth_method: "RPS".to_string(),
            site_name: "user.auth.xboxlive.com".to_string(),
            rps_ticket: format!("d={}", token),
        },
        relying_party: "http://auth.xboxlive.com".to_string(),
        token_type: "JWT".to_string()
    });

    let response = match REQWEST_CLIENT
        .post(XBOX_LIVE_AUTH_URL.clone())
        .headers(headers)
        .json(&body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return Err(XboxLiveError::HTTPRequestError(error));
        }
    };

    if let Err(error) = response.error_for_status_ref() {
        return Err(XboxLiveError::XboxLiveLoginError(error));
    }

    let content: XboxLiveResponse = match response.json().await {
        Ok(content) => content,
        Err(error) => {
            return Err(XboxLiveError::InvalidResponseFormat(error.to_string()));
        }
    };

    if content.display_claims.xui.is_empty() {
        return Err(XboxLiveError::InvalidResponseFormat(
            "No xui claims found".to_string(),
        ));
    }

    Ok(content)
}
