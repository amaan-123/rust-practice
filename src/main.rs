mod db;
mod product_repository;
mod products;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = db::connect(&database_url).await?;

    db::run_migrations(&pool).await?;

    let product = product_repository::create_product(&pool, "Practice Laptop", 10).await?;

    println!("Created: {:?}", product);

    let found = product_repository::get_product(&pool, product.id).await?;
    println!("Retrieved: {:?}", found);

    let missing = product_repository::get_product(&pool, -1).await?;
    println!("Missing ID: {:?}", missing);

    Ok(())
}
