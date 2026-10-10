use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
struct AppState {
    products: Arc<RwLock<HashMap<i64, ProductResponse>>>,
}

#[derive(Deserialize)]
struct CreateProductRequest {
    name: String,
    available_quantity: i32,
}

#[derive(Clone, Serialize)]
struct ProductResponse {
    id: i64,
    name: String,
    available_quantity: i32,
}

#[derive(Deserialize)]
struct CreateReservationRequest {
    quantity: i32,
    request_id: String,
}

#[derive(Serialize)]
struct ReservationResponse {
    product_id: i64,
    quantity: i32,
    request_id: String,
    remaining_quantity: i32,
}

async fn create_product(
    State(state): State<AppState>,
    Json(request): Json<CreateProductRequest>,
) -> (StatusCode, Json<ProductResponse>) {
    let mut products = state.products.write().await;

    let id = products.len() as i64 + 1;

    let product = ProductResponse {
        id,
        name: request.name,
        available_quantity: request.available_quantity,
    };

    products.insert(id, product.clone());

    (StatusCode::CREATED, Json(product))
}

async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ProductResponse>, StatusCode> {
    let products = state.products.read().await;

    match products.get(&id) {
        Some(product) => Ok(Json(product.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_reservation(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<CreateReservationRequest>,
) -> Result<Json<ReservationResponse>, StatusCode> {
    if request.quantity <= 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut products = state.products.write().await;

    let product = products.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if request.quantity > product.available_quantity {
        return Err(StatusCode::CONFLICT);
    }

    product.available_quantity -= request.quantity;

    Ok(Json(ReservationResponse {
        product_id: id,
        quantity: request.quantity,
        request_id: request.request_id,
        remaining_quantity: product.available_quantity,
    }))
}

#[tokio::main]
async fn main() {
    let state = AppState {
        products: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/products", post(create_product))
        .route("/products/{id}", get(get_product))
        .route("/products/{id}/reservations", post(create_reservation))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind TCP listener");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("server failed");
}
