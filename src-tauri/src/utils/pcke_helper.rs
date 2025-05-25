use base64::{prelude::BASE64_URL_SAFE, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

const PKCE_CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-._~";

#[derive(Debug, Clone)]
pub struct PckeChallenge {
    pub code_verifier: String,
    pub code_challenge: String,
}

fn generate_code_verifier() -> String {
    let mut rng = rand::rng();
    (0..128)
        .map(|_| {
            let i = rng.random_range(..PKCE_CHARS.len());
            PKCE_CHARS[i] as char
        })
        .collect()
}

fn generate_code_challenge(code_verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let result = hasher.finalize();

    BASE64_URL_SAFE.encode(result).replace("=", "")
}

pub fn generate_pcke_challenge() -> PckeChallenge {
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(code_verifier.as_str());

    PckeChallenge {
        code_verifier,
        code_challenge,
    }
}
