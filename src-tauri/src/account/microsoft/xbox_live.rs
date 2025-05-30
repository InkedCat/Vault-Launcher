use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::http::HeaderMap;
use thiserror::Error;

use crate::REQWEST_CLIENT;

use super::XBOX_LIVE_AUTH_DOMAIN;

lazy_static! {
    static ref XBOX_LIVE_AUTH_URL: String =
        format!("https://user.{}/user/authenticate", XBOX_LIVE_AUTH_DOMAIN);
    static ref XSTS_AUTH_URL: String =
        format!("https://xsts.{}/xsts/authorize", XBOX_LIVE_AUTH_DOMAIN);
}

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

pub async fn login_to_xbox_live(token: &String) -> Result<XboxLiveResponse, XboxLiveError> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("x-xbl-contract-version", "1".parse().unwrap());

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

    let content = match response.json().await {
        Ok(content) => content,
        Err(error) => {
            return Err(XboxLiveError::InvalidResponseFormat(error.to_string()));
        }
    };

    Ok(content)
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XSTSRequestProperties {
    sandbox_id: String,
    user_tokens: Vec<String>,
}

pub async fn get_xsts_token(xbox_live_token: &str) -> Result<XboxLiveResponse, XboxLiveError> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("x-xbl-contract-version", "1".parse().unwrap());

    let body = json!(XboxLiveTokenRequest {
        properties: XSTSRequestProperties {
            sandbox_id: "RETAIL".to_string(),
            user_tokens: vec![xbox_live_token.to_string()],
        },
        relying_party: "rp://api.minecraftservices.com/".to_string(),
        token_type: "JWT".to_string()
    });

    let response = match REQWEST_CLIENT
        .post(XSTS_AUTH_URL.clone())
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

    let content = match response.json().await {
        Ok(content) => content,
        Err(error) => {
            return Err(XboxLiveError::InvalidResponseFormat(error.to_string()));
        }
    };

    Ok(content)
}
