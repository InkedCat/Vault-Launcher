use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::http::HeaderMap;
use thiserror::Error;

use crate::REQWEST_CLIENT;

use super::MINECRAFT_API_URL;

const MOJANG_PUBLIC_KEY: &str = include_str!("mojang_public.key");

lazy_static! {
    static ref MINECRAFT_AUTH_URL: String =
        format!("{}/authentication/login_with_xbox", MINECRAFT_API_URL);
    static ref MINECRAFT_OWNS_URL: String = format!("{}/entitlements/mcstore", MINECRAFT_API_URL);
    static ref MOJANG_DECODING_KEY: DecodingKey =
        DecodingKey::from_rsa_pem(MOJANG_PUBLIC_KEY.as_bytes())
            .expect("Failed to parse Mojang public key");
    static ref REQUIRED_ENTITLEMENTS: Vec<String> = vec![
        "product_minecraft".to_string(),
        "game_minecraft".to_string(),
    ];
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct MinecraftTokenRequest {
    identity_token: String,
}

#[derive(Error, Debug)]
pub enum MinecraftTokenError {
    #[error("HTTP request error: {0}")]
    HTTPRequestError(reqwest::Error),
    #[error("Minecraft token error: {0}")]
    MinecraftTokenRequestError(reqwest::Error),
    #[error("Failed to parse response: {0}")]
    InvalidResponseFormat(String),
}

#[derive(Deserialize)]
pub struct MinecraftTokenResponse {
    pub username: String,
    pub roles: Vec<String>,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}

pub async fn get_minecraft_token(
    xsts_token: &str,
    user_hash: &str,
) -> Result<MinecraftTokenResponse, MinecraftTokenError> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());

    let body = json!(MinecraftTokenRequest {
        identity_token: format!("XBL3.0 x={};{}", user_hash, xsts_token),
    });

    let response = match REQWEST_CLIENT
        .post(MINECRAFT_AUTH_URL.clone())
        .headers(headers)
        .json(&body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return Err(MinecraftTokenError::HTTPRequestError(error));
        }
    };

    if let Err(error) = response.error_for_status_ref() {
        return Err(MinecraftTokenError::MinecraftTokenRequestError(error));
    }

    let content = match response.json().await {
        Ok(content) => content,
        Err(error) => {
            return Err(MinecraftTokenError::InvalidResponseFormat(
                error.to_string(),
            ));
        }
    };

    Ok(content)
}

#[derive(Deserialize)]
pub struct MinecraftOwnsItem {
    pub name: String,
    pub signature: String,
}

#[derive(Deserialize)]
pub struct MinecraftOwnsResponse {
    pub items: Vec<MinecraftOwnsItem>,
    pub signature: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
}

#[derive(Error, Debug)]
pub enum MinecraftOwnsError {
    #[error("HTTP request error: {0}")]
    HTTPRequestError(reqwest::Error),
    #[error("Minecraft owns error: {0}")]
    MinecraftOwnsRequestError(reqwest::Error),
    #[error("Failed to parse response: {0}")]
    InvalidResponseFormat(String),
}

pub async fn user_own_minecraft(minecraft_token: &str) -> Result<bool, MinecraftOwnsError> {
    let response = match REQWEST_CLIENT
        .get(MINECRAFT_OWNS_URL.clone())
        .bearer_auth(minecraft_token)
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return Err(MinecraftOwnsError::HTTPRequestError(error));
        }
    };

    if let Err(error) = response.error_for_status_ref() {
        return Err(MinecraftOwnsError::MinecraftOwnsRequestError(error));
    }

    let content: MinecraftOwnsResponse = match response.json().await {
        Ok(content) => content,
        Err(error) => {
            return Err(MinecraftOwnsError::InvalidResponseFormat(error.to_string()));
        }
    };

    let minecraft_ownership = match check_minecraft_ownership(content) {
        Ok(ownership) => ownership,
        Err(error) => {
            return Err(MinecraftOwnsError::InvalidResponseFormat(error.to_string()));
        }
    };

    Ok(minecraft_ownership)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    entitlements: Vec<Entitlement>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Entitlement {
    name: String,
}

fn check_minecraft_ownership(
    content: MinecraftOwnsResponse,
) -> Result<bool, jsonwebtoken::errors::Error> {
    let parsed = decode::<Claims>(
        &content.signature,
        &MOJANG_DECODING_KEY,
        &Validation::new(Algorithm::RS256),
    )?;

    if parsed.claims.entitlements.is_empty() {
        return Ok(false);
    }

    Ok(parsed
        .claims
        .entitlements
        .iter()
        .any(|entitlement| REQUIRED_ENTITLEMENTS.contains(&entitlement.name)))
}
