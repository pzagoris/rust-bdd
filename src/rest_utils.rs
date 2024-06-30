//! # Rest utilities module
//!
//! It is a collection of utility functions to assist in making HTTP requests using the reqwest library. 
//! It includes functions for creating and configuring requests, parsing HTTP methods, generating payloads with nonce values, 
//! building URLs, and parsing query strings.

use crate::auth::generate_nonce_value;
use crate::config::get_config_value;
use reqwest::{Body, Client, Method, Request};
use std::collections::HashMap;

/// Creates a `reqwest::Request` instance based on provided parameters.
pub fn create_request(
    client: &Client,
    method: Method,
    url: &str,
    headers: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
    payload: Option<HashMap<String, String>>,
) -> Result<Request, String> {
    let mut request_builder = client.request(method.clone(), url);

    // Set headers
    if let Some(headers) = headers {
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }
    }

    // Set query parameters if provided
    if let Some(query_params) = query_params {
        request_builder = request_builder.query(&query_params);
    }

    // Set payload (form data) if provided, for POST requests
    if method == Method::POST || method == Method::PUT {
        if let Some(payload) = payload {
            // Serialize form data
            let form_data = serde_urlencoded::to_string(payload)
                .map_err(|e| format!("Failed to serialize form data: {}", e))?;

            // Convert form_data into a reqwest::Body
            let body: Body = form_data.into(); // Specify the type here

            // Set the body of the request
            request_builder = request_builder.body(body);
        }
    }

    // Build and return the Request
    request_builder.build().map_err(|e| e.to_string())
}


/// Helper function to identify the HTTP Method type.
pub fn parse_method(method_str: &str) -> Result<Method, String> {
    match method_str.to_uppercase().as_str() {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "PUT" => Ok(Method::PUT),
        "DELETE" => Ok(Method::DELETE),
        "HEAD" => Ok(Method::HEAD),
        "OPTIONS" => Ok(Method::OPTIONS),
        "CONNECT" => Ok(Method::CONNECT),
        "PATCH" => Ok(Method::PATCH),
        "TRACE" => Ok(Method::TRACE),
        _ => Err(format!("Invalid HTTP method: {}", method_str)),
    }
}

/// Creates payload with nonce.
pub fn create_nonce_payload() -> HashMap<String, String> {
    let nonce = generate_nonce_value();
    // Create signature payload
    let mut nonce_payload = HashMap::new();
    nonce_payload.insert("nonce".to_string(), nonce);
    nonce_payload
}

/// Build URL from path.
pub fn build_url(path: &str) -> Result<String, String> {
    let mut url = get_config_value("api_url").map_err(|e| e.to_string())?;
    let path_value = get_config_value(path).map_err(|e| e.to_string())?;
    url = format!("{}{}", url, path_value);
    Ok(url)
}

/// Parses a query string into a hashmap of key-value pairs.
pub fn parse_query_parameters(query_string: &str) -> Result<HashMap<&str, &str>, String> {
    let mut map = HashMap::new();
    for pair in query_string.split('&') {
        if let Some((key, value)) = split_pair(pair) {
            map.insert(key, value);
        } else {
            // Return an error if any pair is malformed
            return Err(format!("Invalid key-value pair: {}", pair));
        }
    }
    Ok(map)
}

// Helper function that is used by parse_query_parameters function
fn split_pair(pair: &str) -> Option<(&str, &str)> {
    if let Some(idx) = pair.find('=') {
        let (key, value) = pair.split_at(idx);
        Some((key, &value[1..]))
    } else {
        None
    }
}
