// Even if you aren't actively calling the previous learning modules 1-10 code in your new `main()` function, keeping the `mod earlier_practice;` declaration at the top of the file is highly recommended. It ensures `cargo check` will continue analyzing your previous code, keeping it valid as you update Rust versions or learn new things.
mod earlier_practice;
use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

// This `handler` returns JSON. Axum converts the response struct into a JSON response because it implements Serialize.
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: String::from("ok"), //hard-coded; eventually use application logic and PostgreSQL
    })
}

#[tokio::main]
async fn main() {
    // Route registration: A `GET /health` request invokes `health_check.
    let app = Router::new().route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind TCP listener");

    println!("Server running at http://127.0.0.1:3000");

    // Start the server:
    // This serves incoming HTTP requests using Tokio.
    axum::serve(listener, app).await.expect("server failed");
    // The expect() calls above are for server startup failures, not routine client-request processing. Later, we'll improve error handling where appropriate.
}
