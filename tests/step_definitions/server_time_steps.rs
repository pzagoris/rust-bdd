use api_tests::api::responses::api_response::ApiResponse;
use api_tests::api::responses::server_time::ServerTime;
use api_tests::api::ApiWorld;
use cucumber::then;

#[then(expr = "validate the response of server time")]
async fn validate_server_time_response(world: &mut ApiWorld) -> Result<(), String> {
    let body_guard = world.last_response_body.lock().await;
    if let Some(body) = &*body_guard {
        let response: ApiResponse<ServerTime> = serde_json::from_value(body.clone())
            .map_err(|e| format!("Failed to deserialize ServerTime: {}", e.to_string()))?;
        assert!(response.error.is_empty());
        assert!(response.result.is_some());
    }
    Ok(())
}
