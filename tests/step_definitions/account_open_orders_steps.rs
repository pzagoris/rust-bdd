use api_tests::api::responses::account_open_orders::AccountOpenOrders;
use api_tests::api::responses::api_response::ApiResponse;
use api_tests::api::update_world_with_response;
use api_tests::api::ApiWorld;
use api_tests::auth::compute_signature;
use api_tests::config::get_config_value;
use api_tests::rest_utils::create_nonce_payload;
use api_tests::rest_utils::create_request;
use cucumber::{given, then};
use reqwest::Method;
use std::collections::HashMap;

#[given(expr = "POST request to retrieve account open orders")]
async fn get_account_open_orders(world: &mut ApiWorld) -> Result<(), String> {
    // Fetch API key and other configuration values
    let api_key = std::env::var("API_KEY").map_err(|e| format!("Failed to get API_KEY: {}", e))?;
    let api_url =
        get_config_value("api_url").map_err(|e| format!("Failed to get api_url: {}", e))?;
    let openorders_path = get_config_value("openorders_path")
        .map_err(|e| format!("Failed to get openorders_path: {}", e))?;
    // Construct URL
    let url = format!("{}{}", api_url, openorders_path);

    // Create signature payload
    let nonce_payload = create_nonce_payload();

    let nonce_payload_str: HashMap<&str, &str> = nonce_payload
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    // Compute signature
    let signature = compute_signature(&openorders_path, &nonce_payload_str)
        .map_err(|e| format!("Failed to compute signature: {}", e))?;

    // Prepare headers
    let mut headers = HashMap::new();
    headers.insert("API-Key".to_string(), api_key);
    headers.insert("API-Sign".to_string(), signature.to_owned());

    // Clone the client from the world context
    let client = world.client.clone();

    // Create and execute request
    let request = create_request(
        &client,
        Method::POST,
        &url,
        Some(headers),
        None,
        Some(nonce_payload),
    )
    .map_err(|e| format!("Failed to create request: {}", e))?;

    let response = client
        .execute(request)
        .await
        .map_err(|e| format!("Failed to execute request: {}", e))?;

    // Update world state with the response and signature
    update_world_with_response(world, response).await?;
    *world.signature.lock().await = signature;

    Ok(())
}

#[then(expr = "the account open orders are {int}")]
async fn validate_open_orders_response(
    world: &mut ApiWorld,
    numbers_of_orders: usize,
) -> Result<(), String> {
    let body_guard = world.last_response_body.lock().await;
    if let Some(body) = &*body_guard {
        let response: ApiResponse<AccountOpenOrders> = serde_json::from_value(body.clone())
            .map_err(|e| format!("Failed to deserialize AccountOpenOrders: {}", e.to_string()))?;
        assert!(response.error.is_empty());
        assert!(response.result.is_some());
        assert_eq!(response.result.unwrap().open.len(), numbers_of_orders);
    }
    Ok(())
}
