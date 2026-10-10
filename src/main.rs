mod db;
mod product_repository;
mod products;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use sqlx::PgPool;
use std::sync::Arc;

use products::{CreateProductRequest, Product};

type AppState = Arc<PgPool>;

async fn create_product(
    State(pool): State<AppState>,
    Json(request): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    if request.name.trim().is_empty() || request.available_quantity < 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let product =
        product_repository::create_product(&pool, &request.name, request.available_quantity)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(product)))
}

async fn get_product(
    State(pool): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, StatusCode> {
    let product = product_repository::get_product(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match product {
        Some(product) => Ok(Json(product)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = db::connect(&database_url).await?;

    db::run_migrations(&pool).await?;

    let app = Router::new()
        .route("/products", post(create_product))
        .route("/products/{id}", get(get_product))
        .with_state(Arc::new(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
