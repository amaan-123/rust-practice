// Even if you aren't actively calling the old learning parts 1-10 code in your new `main()` function, keeping the `mod earlier_practice;` declaration at the top of the file is highly recommended. It ensures `cargo check` will continue analyzing your old code, keeping it valid as you update Rust versions or learn new things.
mod earlier_practice;
use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: String::from("ok"),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind TCP listener");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("server failed");
}
