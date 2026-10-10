use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub available_quantity: i32,
}

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub available_quantity: i32,
}
