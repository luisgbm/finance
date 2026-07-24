use sqlx::{SqliteConnection, SqlitePool};

use crate::error::AppError;
use crate::models::{NewScheduledTransaction, ScheduledTransaction};

const COLUMNS: &str = "id, kind, value, description, created_date, account_id, category_id, \
    origin_account_id, destination_account_id, repeat, repeat_freq, repeat_interval, \
    infinite_repeat, end_after_repeats, current_repeat_count, next_date";

pub async fn insert(
    pool: &SqlitePool,
    new: &NewScheduledTransaction,
) -> Result<ScheduledTransaction, AppError> {
    let st = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "INSERT INTO scheduled_transactions \
         (kind, value, description, created_date, account_id, category_id, origin_account_id, \
          destination_account_id, repeat, repeat_freq, repeat_interval, infinite_repeat, \
          end_after_repeats, current_repeat_count, next_date) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
         RETURNING {COLUMNS}"
    ))
    .bind(new.kind)
    .bind(new.value)
    .bind(new.description.as_deref())
    .bind(new.created_date)
    .bind(new.account_id)
    .bind(new.category_id)
    .bind(new.origin_account_id)
    .bind(new.destination_account_id)
    .bind(new.repeat)
    .bind(new.repeat_freq)
    .bind(new.repeat_interval)
    .bind(new.infinite_repeat)
    .bind(new.end_after_repeats)
    .bind(new.current_repeat_count)
    .bind(new.next_date)
    .fetch_one(pool)
    .await?;

    Ok(st)
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<ScheduledTransaction>, AppError> {
    let rows = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "SELECT {COLUMNS} FROM scheduled_transactions ORDER BY created_date DESC"
    ))
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get(pool: &SqlitePool, id: i32) -> Result<ScheduledTransaction, AppError> {
    let st = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "SELECT {COLUMNS} FROM scheduled_transactions WHERE id = ?"
    ))
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(st)
}

pub async fn update(
    pool: &SqlitePool,
    id: i32,
    new: &NewScheduledTransaction,
) -> Result<ScheduledTransaction, AppError> {
    let mut conn = pool.acquire().await?;
    update_on(&mut conn, id, new).await
}

/// Update a scheduled transaction on the caller's connection/transaction, so paying a
/// repeating schedule can advance it atomically with materialising the payment.
pub async fn update_on(
    conn: &mut SqliteConnection,
    id: i32,
    new: &NewScheduledTransaction,
) -> Result<ScheduledTransaction, AppError> {
    let st = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "UPDATE scheduled_transactions SET \
            kind = ?, value = ?, description = ?, created_date = ?, account_id = ?, \
            category_id = ?, origin_account_id = ?, destination_account_id = ?, repeat = ?, \
            repeat_freq = ?, repeat_interval = ?, infinite_repeat = ?, end_after_repeats = ?, \
            current_repeat_count = ?, next_date = ? \
         WHERE id = ? RETURNING {COLUMNS}"
    ))
    .bind(new.kind)
    .bind(new.value)
    .bind(new.description.as_deref())
    .bind(new.created_date)
    .bind(new.account_id)
    .bind(new.category_id)
    .bind(new.origin_account_id)
    .bind(new.destination_account_id)
    .bind(new.repeat)
    .bind(new.repeat_freq)
    .bind(new.repeat_interval)
    .bind(new.infinite_repeat)
    .bind(new.end_after_repeats)
    .bind(new.current_repeat_count)
    .bind(new.next_date)
    .bind(id)
    .fetch_one(&mut *conn)
    .await?;

    Ok(st)
}

pub async fn delete(pool: &SqlitePool, id: i32) -> Result<ScheduledTransaction, AppError> {
    let mut conn = pool.acquire().await?;
    delete_on(&mut conn, id).await
}

/// Delete a scheduled transaction on the caller's connection/transaction, so paying a
/// one-off schedule can remove it atomically with materialising the payment.
pub async fn delete_on(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<ScheduledTransaction, AppError> {
    let st = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "DELETE FROM scheduled_transactions WHERE id = ? RETURNING {COLUMNS}"
    ))
    .bind(id)
    .fetch_one(&mut *conn)
    .await?;

    Ok(st)
}
