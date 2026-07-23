use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::{NewScheduledTransaction, ScheduledTransaction};

const COLUMNS: &str = "id, kind, value, description, created_date, account_id, category_id, \
    origin_account_id, destination_account_id, repeat, repeat_freq, repeat_interval, \
    infinite_repeat, end_after_repeats, current_repeat_count, next_date, user_id";

pub async fn insert(
    pool: &SqlitePool,
    new: &NewScheduledTransaction,
) -> Result<ScheduledTransaction, AppError> {
    let st = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "INSERT INTO scheduled_transactions \
         (kind, value, description, created_date, account_id, category_id, origin_account_id, \
          destination_account_id, repeat, repeat_freq, repeat_interval, infinite_repeat, \
          end_after_repeats, current_repeat_count, next_date, user_id) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
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
    .bind(new.user_id)
    .fetch_one(pool)
    .await?;

    Ok(st)
}

pub async fn get_all(
    pool: &SqlitePool,
    user_id: i32,
) -> Result<Vec<ScheduledTransaction>, AppError> {
    let rows = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "SELECT {COLUMNS} FROM scheduled_transactions WHERE user_id = ? ORDER BY created_date DESC"
    ))
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get(
    pool: &SqlitePool,
    id: i32,
    user_id: i32,
) -> Result<ScheduledTransaction, AppError> {
    let st = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "SELECT {COLUMNS} FROM scheduled_transactions WHERE user_id = ? AND id = ?"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(st)
}

pub async fn update(
    pool: &SqlitePool,
    id: i32,
    new: &NewScheduledTransaction,
    user_id: i32,
) -> Result<ScheduledTransaction, AppError> {
    let st = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "UPDATE scheduled_transactions SET \
            kind = ?, value = ?, description = ?, created_date = ?, account_id = ?, \
            category_id = ?, origin_account_id = ?, destination_account_id = ?, repeat = ?, \
            repeat_freq = ?, repeat_interval = ?, infinite_repeat = ?, end_after_repeats = ?, \
            current_repeat_count = ?, next_date = ? \
         WHERE user_id = ? AND id = ? RETURNING {COLUMNS}"
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
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(st)
}

pub async fn delete(
    pool: &SqlitePool,
    id: i32,
    user_id: i32,
) -> Result<ScheduledTransaction, AppError> {
    let st = sqlx::query_as::<_, ScheduledTransaction>(&format!(
        "DELETE FROM scheduled_transactions WHERE user_id = ? AND id = ? RETURNING {COLUMNS}"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(st)
}
