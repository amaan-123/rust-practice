use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct CreateReservationRequest {
    pub quantity: i32,
    pub request_id: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Reservation {
    pub id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub request_id: String,
}

#[derive(Debug)]
pub enum ReservationError {
    InvalidQuantity,
    ProductNotFound,
    InsufficientInventory,
    IdempotencyConflict,
    Database(sqlx::Error),
}
