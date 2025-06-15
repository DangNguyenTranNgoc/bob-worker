use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod db;
mod routes;

#[tokio::main]
async fn main() {
    // Init dotenv
    dotenv().ok();
    // Initialize tracing subscriber with file logging
    let file_appender = tracing_appender::rolling::daily("logs", "bob_api");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_level(true)
        .init();

    // Initialize database pool
    let db = db::init_db().await.expect("Failed to init DB");
    println!("Database initialized successfully");
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/send-message", post(routes::send_message::handler))
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6996").await.unwrap();
    info!("Starting server at http://0.0.0.0:6996");
    axum::serve(listener, app).await.unwrap();
}
