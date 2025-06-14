use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use dotenv::dotenv;
use reqwest::Client;
use serde_json::json;
use sqlx::SqlitePool;
use std::env;
use tracing::info;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SendMessagePayload {
    pub r#type: String,
    pub name: String,
    pub message: String,
}

pub async fn handler(
    headers: HeaderMap,
    State(_db): State<SqlitePool>,
    Json(payload): Json<SendMessagePayload>,
) -> impl IntoResponse {
    println!("Received request to send message");
    // Load environment variables from .env file
    dotenv().ok(); 
    // Environment variable for auth key
    let auth_key = env::var("API_AUTH_KEY").expect("API_AUTH_KEY not set in .env");
    let url = env::var("API_URL").expect("API_URL not set in .env");
    println!("Using auth key: {}", auth_key);
    println!("Using URL: {}", url);

    let body = json!(payload);

    // Call API here (stubbed for now)
    let client = Client::new();
    let post_result = client
        .post(format!("{}?apikey={}", url, auth_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match post_result {
        Ok(res) if res.status().is_success() => {
            info!("Message sent to channel {}", payload.name);
            (StatusCode::OK, "Message sent".to_string())
        }
        Ok(res) => {
            let status = res.status();
            let err_text = res.text().await.unwrap_or_default();
            let message = format!("Failed to send message: {} - {}", status, err_text);
            info!("{}", &message);
            (status, message)
        }
        Err(e) => {
            info!("Error sending message: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send message".to_string(),
            )
        }
    }
}
