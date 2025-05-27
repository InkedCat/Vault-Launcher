use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::http::HeaderMap;

lazy_static! {
    static ref XBOX_LIVE_AUTH_URL: String =
        format!("{}/user/authenticate", crate::account::XBOX_LIVE_URL);
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct XboxLiveProperties {
    auth_method: String,
    site_name: String,
    rps_ticket: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct XboxLiveRequest {
    properties: XboxLiveProperties,
    relying_party: String,
    token_type: String,
}

#[derive(Deserialize, Debug)]
struct Xui {
    uhs: String,
}

#[derive(Deserialize, Debug)]
pub struct DisplayClaims {
    xui: Vec<Xui>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveResponse {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: DisplayClaims,
}

pub async fn login_to_xbox_live(token: &String) -> Result<XboxLiveResponse, anyhow::Error> {
    let client = reqwest::Client::new();

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("x-xbl-contract-version", "1".parse().unwrap());

    let body = json!(XboxLiveRequest {
        properties: XboxLiveProperties {
            auth_method: "RPS".to_string(),
            site_name: "user.auth.xboxlive.com".to_string(),
            rps_ticket: format!("d={}", token)
        },
        relying_party: "http://auth.xboxlive.com".to_string(),
        token_type: "JWT".to_string()
    });

    let response = client
        .post(XBOX_LIVE_AUTH_URL.clone())
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    response.error_for_status_ref()?;
    let content: XboxLiveResponse = response.json().await?;

    Ok(content)
}
