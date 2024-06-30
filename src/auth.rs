//! # Authorization module
//!
//! Contains functionality related to the authorization.
//!
//! Important note: The two-factor authentication (2FA) is not implemented.

use base64::engine::general_purpose;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
type HmacSha512 = Hmac<sha2::Sha512>;

/// Computes the signature as a base64-encoded string.
pub fn compute_signature(url_path: &str, values: &HashMap<&str, &str>) -> Result<String, String> {
    let api_secret =
        std::env::var("API_SECRET").map_err(|e| format!("Failed to read `API_SECRE` : {}", e))?;
    let b64_decoded_secret = decode_base64(&api_secret)?;

    let payload = construct_payload(values);
    let shasum = compute_sha256(values.get("nonce").ok_or("Nonce not found")?, &payload);
    let macsum = compute_hmac_sha512(url_path, &shasum, &b64_decoded_secret)?;
    Ok(general_purpose::STANDARD.encode(macsum))
}

/// Genarate the nonce value by getting the current timestamp in milliseconds.
pub fn generate_nonce_value() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string()
}

/// Constructs the payload string from a HashMap of key-value pairs.
fn construct_payload(values: &HashMap<&str, &str>) -> String {
    let mut payload = String::new();
    for (key, value) in values {
        payload.push_str(&format!("{}={}&", key, value));
    }
    payload.pop(); // Remove the trailing '&'
    payload
}

/// Decodes a Base64 encoded string into a vector of bytes.
fn decode_base64(secret: &str) -> Result<Vec<u8>, String> {
    let decoded_secret = STANDARD
        .decode(secret)
        .map_err(|e| format!("Failed to decode: {} ", e))?;
    Ok(decoded_secret)
}

/// Computes the SHA-256 hash of a nonce and payload string.
fn compute_sha256(nonce: &str, payload: &str) -> [u8; 32] {
    let mut sha256 = Sha256::new();
    sha256.update(nonce.as_bytes());
    sha256.update(payload.as_bytes());
    sha256.finalize().into()
}

/// Computes the HMAC-SHA-512 hash of a URL path, SHA-256 hash, and secret key.
fn compute_hmac_sha512(url_path: &str, shasum: &[u8], secret: &[u8]) -> Result<[u8; 64], String> {
    let mut mac = HmacSha512::new_from_slice(secret)
        .map_err(|e| format!("Failed to compute hmac-sha512 : {}", e))?;
    mac.update(url_path.as_bytes());
    mac.update(shasum);
    Ok(mac.finalize().into_bytes().into())
}
