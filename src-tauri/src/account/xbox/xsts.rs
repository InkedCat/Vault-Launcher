use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::http::HeaderMap;

use crate::REQWEST_CLIENT;

use super::{live::XboxLiveError, XSTS_AUTH_URL};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XSTSRequest {
    properties: XSTSRequestProperties,
    relying_party: String,
    token_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct XSTSRequestProperties {
    sandbox_id: String,
    user_tokens: Vec<String>,
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
pub struct XSTSResponse {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: DisplayClaims,
}

pub async fn get_xsts_token(xbox_live_token: &str) -> Result<XSTSResponse, XboxLiveError> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("x-xbl-contract-version", "1".parse().unwrap());

    let body = json!(XSTSRequest {
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

    let content: XSTSResponse = match response.json().await {
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
