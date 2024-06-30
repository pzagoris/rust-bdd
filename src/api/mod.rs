use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod responses;

/// World struct for storing API testing state.
#[derive(Debug, Default, cucumber::World)]
pub struct ApiWorld {
    /// Shared HTTP client instance wrapped in an `Arc`.
    pub client: Arc<reqwest::Client>,
    /// Last Api response status.
    pub last_response_status: Arc<Mutex<Option<u16>>>,
    /// Last Api response headers
    pub last_response_headers: Arc<Mutex<Option<reqwest::header::HeaderMap>>>,
    /// Last Api response body.
    pub last_response_body: Arc<Mutex<Option<serde_json::Value>>>,
    /// Signature values used in requests (private api).
    pub signature: Arc<Mutex<String>>,
}

/// Helper function to update the world object with response details.
pub async fn update_world_with_response(
    world: &mut ApiWorld,
    response: reqwest::Response,
) -> Result<(), String> {
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let body = response.json::<Value>().await.map_err(|e| e.to_string())?;
    *world.last_response_status.lock().await = Some(status);
    *world.last_response_headers.lock().await = Some(headers);
    *world.last_response_body.lock().await = Some(body);
    Ok(())
}
