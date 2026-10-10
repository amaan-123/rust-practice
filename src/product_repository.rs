use sqlx::PgPool;

use crate::products::Product;

pub async fn create_product(
    pool: &PgPool,
    name: &str,
    available_quantity: i32,
) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (name, available_quantity)
        VALUES ($1, $2)
        RETURNING id, name, available_quantity
        "#,
    )
    .bind(name)
    .bind(available_quantity)
    .fetch_one(pool)
    .await
}

pub async fn get_product(pool: &PgPool, id: i64) -> Result<Option<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        SELECT id, name, available_quantity
        FROM products
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
