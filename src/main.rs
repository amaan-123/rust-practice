// Even if you aren't actively calling the previous learning modules 1-10 code in your new `main()` function, keeping the `mod earlier_practice;` declaration at the top of the file is highly recommended. It ensures `cargo check` will continue analyzing your previous code, keeping it valid as you update Rust versions or learn new things.
mod earlier_practice;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateProductRequest {
    name: String,
    available_quantity: i32,
}

#[derive(Serialize)]
struct ProductResponse {
    id: i64,
    name: String,
    available_quantity: i32,
}

async fn create_product(
    Json(request): Json<CreateProductRequest>,
) -> (StatusCode, Json<ProductResponse>) {
    let product = ProductResponse {
        id: 1, // One deliberate limitation: Every request currently returns ID 1, and nothing is persisted. We'll replace this behavior with database-backed logic.
        name: request.name, // The String moves from the request into the response struct. We don't need to clone it.
        available_quantity: request.available_quantity,
    };

    (StatusCode::CREATED, Json(product))
}

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/products", post(create_product));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind TCP listener");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("server failed");
}
