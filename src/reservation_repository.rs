use sqlx::PgPool;

use crate::reservations::{Reservation, ReservationError};

fn is_database_error(error: &sqlx::Error, code: &str) -> bool {
    error
        .as_database_error()
        .and_then(|db_error| db_error.code())
        .as_deref()
        == Some(code)
}

pub async fn create_reservation(
    pool: &PgPool,
    product_id: i64,
    quantity: i32,
    request_id: &str,
) -> Result<Reservation, ReservationError> {
    if quantity <= 0 {
        return Err(ReservationError::InvalidQuantity);
    }

    let mut tx = pool.begin().await.map_err(ReservationError::Database)?;

    // Claim the idempotency key before changing inventory.
    let insert_result = sqlx::query_as::<_, Reservation>(
        r#"
        INSERT INTO reservations (product_id, quantity, request_id)
        VALUES ($1, $2, $3)
        RETURNING id, product_id, quantity, request_id
        "#,
    )
    .bind(product_id)
    .bind(quantity)
    .bind(request_id)
    .fetch_one(&mut *tx)
    .await;

    let reservation = match insert_result {
        Ok(reservation) => reservation,

        Err(error) if is_database_error(&error, "23505") => {
            tx.rollback().await.map_err(ReservationError::Database)?;

            let existing = sqlx::query_as::<_, Reservation>(
                r#"
                SELECT id, product_id, quantity, request_id
                FROM reservations
                WHERE request_id = $1
                "#,
            )
            .bind(request_id)
            .fetch_optional(pool)
            .await
            .map_err(ReservationError::Database)?;

            return match existing {
                Some(existing)
                    if existing.product_id == product_id && existing.quantity == quantity =>
                {
                    Ok(existing)
                }
                Some(_) => Err(ReservationError::IdempotencyConflict),
                None => Err(ReservationError::Database(error)),
            };
        }

        Err(error) if is_database_error(&error, "23503") => {
            tx.rollback().await.map_err(ReservationError::Database)?;

            return Err(ReservationError::ProductNotFound);
        }

        Err(error) => {
            tx.rollback().await.map_err(ReservationError::Database)?;

            return Err(ReservationError::Database(error));
        }
    };

    // Deduct inventory atomically, only if sufficient stock remains.
    let updated_product = sqlx::query_scalar::<_, i64>(
        r#"
        UPDATE products
        SET available_quantity = available_quantity - $1
        WHERE id = $2
          AND available_quantity >= $1
        RETURNING id
        "#,
    )
    .bind(quantity)
    .bind(product_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(ReservationError::Database)?;

    if updated_product.is_none() {
        tx.rollback().await.map_err(ReservationError::Database)?;

        return Err(ReservationError::InsufficientInventory);
    }

    tx.commit().await.map_err(ReservationError::Database)?;

    Ok(reservation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn test_pool() -> PgPool {
        dotenvy::dotenv().ok();

        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for database tests");

        let pool = PgPool::connect(&database_url)
            .await
            .expect("failed to connect to test database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("failed to run migrations");

        pool
    }

    async fn create_test_product(pool: &PgPool, quantity: i32) -> i64 {
        sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO products (name, available_quantity)
            VALUES ('Test product', $1)
            RETURNING id
            "#,
        )
        .bind(quantity)
        .fetch_one(pool)
        .await
        .expect("failed to create test product")
    }

    #[tokio::test]
    async fn reservation_decreases_inventory() {
        let pool = test_pool().await;
        let product_id = create_test_product(&pool, 10).await;

        let result = create_reservation(&pool, product_id, 3, "test-success-005").await;

        assert!(result.is_ok());

        let remaining: i32 =
            sqlx::query_scalar("SELECT available_quantity FROM products WHERE id = $1")
                .bind(product_id)
                .fetch_one(&pool)
                .await
                .expect("failed to read inventory");

        assert_eq!(remaining, 7);
    }

    #[tokio::test]
    async fn insufficient_inventory_does_not_decrease_stock() {
        let pool = test_pool().await;
        let product_id = create_test_product(&pool, 2).await;

        let result = create_reservation(&pool, product_id, 3, "test-insufficient-001").await;

        assert!(matches!(
            result,
            Err(ReservationError::InsufficientInventory)
        ));

        let remaining: i32 =
            sqlx::query_scalar("SELECT available_quantity FROM products WHERE id = $1")
                .bind(product_id)
                .fetch_one(&pool)
                .await
                .expect("failed to read inventory");

        assert_eq!(remaining, 2);
    }
}
