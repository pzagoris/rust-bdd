use crate::ApiWorld;
use api_tests::api::update_world_with_response;
use api_tests::rest_utils::{build_url, create_request, parse_method};
use cucumber::{given, then};
use std::collections::HashMap;

#[given(expr = "{string} request to {string} path of public api")]
async fn make_request_to_public_api(
    world: &mut ApiWorld,
    method: String,
    path: String,
) -> Result<(), String> {
    let url = build_url(&path)?;
    let client = world.client.clone();
    let request_method = parse_method(&method)?;
    let request = create_request(&client, request_method, &url, None, None, None)?;
    let response = client.execute(request).await.map_err(|e| e.to_string())?;
    update_world_with_response(world, response).await?;
    Ok(())
}

#[given(expr = "{string} request to {string} path of public api with param {word}={word}")]
async fn make_request_with_one_parameter(
    world: &mut ApiWorld,
    method: String,
    path: String,
    key_str: String,
    value_str: String,
) -> Result<(), String> {
    let url = build_url(&path)?;
    let client = world.client.clone();
    let request_method = parse_method(&method)?;
    let mut query_parameters: HashMap<String, String> = HashMap::new();
    query_parameters.insert(key_str, value_str);
    let request = create_request(
        &client,
        request_method,
        &url,
        None,
        Some(query_parameters),
        None,
    )?;
    let response = client.execute(request).await.map_err(|e| e.to_string())?;
    update_world_with_response(world, response).await?;
    Ok(())
}

#[then(expr = "the response status should be {int}")]
async fn check_response_status(world: &mut ApiWorld, expected_status: u16) -> Result<(), String> {
    let status_guard = world.last_response_status.lock().await;
    if let Some(status) = *status_guard {
        assert_eq!(status, expected_status);
    } else {
        return Err("No response foound in the world".into());
    }
    Ok(())
}
