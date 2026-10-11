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
