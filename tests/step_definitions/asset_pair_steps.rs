use api_tests::api::responses::api_response::ApiResponse;
use api_tests::api::responses::asset_pair::AssetPair;
use api_tests::api::ApiWorld;
use cucumber::then;
use std::collections::HashMap;

#[then(expr = "validate the Asset Pairs response contains {word} info")]
async fn validate_assait_pair_response(
    world: &mut ApiWorld,
    expected_pair: String,
) -> Result<(), String> {
    let body_guard = world.last_response_body.lock().await;
    if let Some(body) = &*body_guard {
        let response: ApiResponse<HashMap<String, AssetPair>> =
            serde_json::from_value(body.clone())
                .map_err(|e| format!("Failed to deserialize AssaitPair: {}, ", e.to_string()))?;
        assert!(response.error.is_empty());
        assert!(response.result.is_some());
        // no problem to unwrap in this step, because if code arrive here
        // the response.result would contain data.
        assert!(response.result.unwrap().contains_key(&expected_pair));
    }
    Ok(())
}
