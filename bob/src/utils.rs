use axum::http::{HeaderMap, StatusCode};
use dotenv::dotenv;
use std::env;

/// Extracts and validates the API key from headers using the DB pool.
pub async fn require_valid_key(headers: &HeaderMap) -> Result<(), (StatusCode, &'static str)> {
    // Load environment variables from .env file
    dotenv().ok();
    // Environment variable for auth key
    let api_key = env::var("API_SECRET_KEY").expect("API_SECRET_KEY not set in .env");
    let Some(auth_header) = headers.get("authorization") else {
        return Err((StatusCode::UNAUTHORIZED, "Missing auth key"));
    };

    // Get the key from the header
    let key = auth_header
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid auth key"))?;
    // Check if the key matches the expected API key
    if key != api_key {
        return Err((StatusCode::UNAUTHORIZED, "Invalid auth key"));
    }
    // If the key is valid, return Ok
    tracing::info!("Valid API key provided");

    Ok(())
}
