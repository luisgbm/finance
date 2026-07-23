use chrono::NaiveDateTime;
use sqlx::{SqliteConnection, SqlitePool};

use crate::error::AppError;
use crate::models::{NewTransferData, Transfer};

const COLUMNS: &str =
    "id, origin_account, destination_account, value, description, date, user_id";

/// A transfer leaving an account (origin = the account being viewed), joined with the
/// origin account name.
#[derive(sqlx::FromRow)]
pub struct TransferFromRow {
    pub id: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub origin_account: i32,
    pub origin_name: String,
}

/// A transfer entering an account (destination = the account being viewed), joined with
/// both the origin and destination account names.
#[derive(sqlx::FromRow)]
pub struct TransferToRow {
    pub id: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub origin_account: i32,
    pub origin_name: String,
    pub destination_account: i32,
    pub dest_name: String,
}

pub async fn insert(pool: &SqlitePool, new: &NewTransferData) -> Result<Transfer, AppError> {
    let mut tx = pool.begin().await?;
    let transfer = insert_on(&mut tx, new).await?;
    tx.commit().await?;
    Ok(transfer)
}

/// Insert a transfer using the caller's connection/transaction, so it can be composed
/// atomically with other writes (see [`crate::db::transactions::insert_on`]). Shares the
/// transactions/transfers id space via `seq_tx_tr`.
pub async fn insert_on(
    conn: &mut SqliteConnection,
    new: &NewTransferData,
) -> Result<Transfer, AppError> {
    let id: i64 = sqlx::query_scalar("INSERT INTO seq_tx_tr DEFAULT VALUES RETURNING id")
        .fetch_one(&mut *conn)
        .await?;

    let transfer = sqlx::query_as::<_, Transfer>(&format!(
        "INSERT INTO transfers (id, origin_account, destination_account, value, description, date, user_id) \
         VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING {COLUMNS}"
    ))
    .bind(id)
    .bind(new.origin_account)
    .bind(new.destination_account)
    .bind(new.value)
    .bind(new.description.as_str())
    .bind(new.date)
    .bind(new.user_id)
    .fetch_one(&mut *conn)
    .await?;

    Ok(transfer)
}

pub async fn get(pool: &SqlitePool, id: i32, user_id: i32) -> Result<Transfer, AppError> {
    let transfer = sqlx::query_as::<_, Transfer>(&format!(
        "SELECT {COLUMNS} FROM transfers WHERE user_id = ? AND id = ?"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(transfer)
}

pub async fn get_from_account_joined(
    pool: &SqlitePool,
    account_id: i32,
    user_id: i32,
) -> Result<Vec<TransferFromRow>, AppError> {
    let rows = sqlx::query_as::<_, TransferFromRow>(
        "SELECT tr.id, tr.value, tr.description, tr.date, tr.origin_account, o.name AS origin_name \
         FROM transfers tr JOIN accounts o ON o.id = tr.origin_account \
         WHERE tr.user_id = ? AND tr.origin_account = ?",
    )
    .bind(user_id)
    .bind(account_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_to_account_joined(
    pool: &SqlitePool,
    account_id: i32,
    user_id: i32,
) -> Result<Vec<TransferToRow>, AppError> {
    let rows = sqlx::query_as::<_, TransferToRow>(
        "SELECT tr.id, tr.value, tr.description, tr.date, \
            tr.origin_account, o.name AS origin_name, \
            tr.destination_account, d.name AS dest_name \
         FROM transfers tr \
         JOIN accounts o ON o.id = tr.origin_account \
         JOIN accounts d ON d.id = tr.destination_account \
         WHERE tr.user_id = ? AND tr.destination_account = ?",
    )
    .bind(user_id)
    .bind(account_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn update(
    pool: &SqlitePool,
    id: i32,
    new: &NewTransferData,
    user_id: i32,
) -> Result<Transfer, AppError> {
    let transfer = sqlx::query_as::<_, Transfer>(&format!(
        "UPDATE transfers SET origin_account = ?, destination_account = ?, value = ?, \
            description = ?, date = ? \
         WHERE user_id = ? AND id = ? RETURNING {COLUMNS}"
    ))
    .bind(new.origin_account)
    .bind(new.destination_account)
    .bind(new.value)
    .bind(new.description.as_str())
    .bind(new.date)
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(transfer)
}

pub async fn delete(pool: &SqlitePool, id: i32, user_id: i32) -> Result<Transfer, AppError> {
    let transfer = sqlx::query_as::<_, Transfer>(&format!(
        "DELETE FROM transfers WHERE user_id = ? AND id = ? RETURNING {COLUMNS}"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(transfer)
}
