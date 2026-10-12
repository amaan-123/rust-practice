mod db;
mod product_repository;
mod products;
mod reservation_repository;
mod reservations;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use products::{CreateProductRequest, Product};
use reservations::{CreateReservationRequest, Reservation};
use sqlx::PgPool;
use std::sync::Arc;

type AppState = Arc<PgPool>;

fn build_app(pool: PgPool) -> Router {
    Router::new()
        .route("/products", post(create_product))
        .route("/products/{id}", get(get_product))
        .route("/products/{id}/reservations", post(create_reservation))
        .with_state(Arc::new(pool))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = db::connect(&database_url).await?;

    db::run_migrations(&pool).await?;

    let app = build_app(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

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

async fn create_reservation(
    State(pool): State<AppState>,
    Path(product_id): Path<i64>,
    Json(request): Json<CreateReservationRequest>,
) -> Result<(StatusCode, Json<Reservation>), StatusCode> {
    let reservation = reservation_repository::create_reservation(
        &pool,
        product_id,
        request.quantity,
        &request.request_id,
    )
    .await
    .map_err(|error| match error {
        reservations::ReservationError::InvalidQuantity => StatusCode::BAD_REQUEST,
        reservations::ReservationError::ProductNotFound => StatusCode::NOT_FOUND,
        reservations::ReservationError::InsufficientInventory => StatusCode::CONFLICT,
        reservations::ReservationError::IdempotencyConflict => StatusCode::CONFLICT,
        reservations::ReservationError::Database(error) => {
            eprintln!("Database error: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    })?;

    Ok((StatusCode::CREATED, Json(reservation)))
}

#[cfg(test)]
mod api_tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn create_product_returns_201_and_json() {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = db::connect(&database_url)
            .await
            .expect("database connection failed");

        db::run_migrations(&pool).await.expect("migration failed");

        let app = build_app(pool);

        let request = axum::http::Request::builder()
            .method("POST")
            .uri("/products")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"name":"API Test Product","available_quantity":5}"#,
            ))
            .unwrap();

        let response = app.oneshot(request).await.expect("request failed");

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).expect("invalid JSON response");

        assert_eq!(json["name"], "API Test Product");
        assert_eq!(json["available_quantity"], 5);
        assert!(json["id"].as_i64().is_some());
    }
}
