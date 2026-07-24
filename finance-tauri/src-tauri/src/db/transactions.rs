use chrono::NaiveDateTime;
use sqlx::{SqliteConnection, SqlitePool};

use crate::error::AppError;
use crate::models::{CategoryTypes, NewTransactionData, Transaction};

const COLUMNS: &str = "id, value, description, date, account, category";

/// A transaction joined with its category and account, used to build the
/// `TransactionTransferJoined` response.
#[derive(sqlx::FromRow)]
pub struct TxJoinRow {
    pub id: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub category_id: i32,
    pub category_type: CategoryTypes,
    pub category_name: String,
    pub account_id: i32,
    pub account_name: String,
}

const JOIN_SELECT: &str = "SELECT t.id, t.value, t.description, t.date, \
    t.category AS category_id, c.categorytype AS category_type, c.name AS category_name, \
    t.account AS account_id, a.name AS account_name \
    FROM transactions t \
    JOIN categories c ON c.id = t.category \
    JOIN accounts a ON a.id = t.account";

pub async fn insert(pool: &SqlitePool, new: &NewTransactionData) -> Result<Transaction, AppError> {
    let mut tx = pool.begin().await?;
    let transaction = insert_on(&mut tx, new).await?;
    tx.commit().await?;
    Ok(transaction)
}

/// Insert a transaction using the caller's connection/transaction, so it can be composed
/// atomically with other writes (e.g. paying a scheduled transaction, which also advances
/// or deletes the schedule in the same transaction).
///
/// Transactions and transfers share a single id space (React uses the id as a list key when
/// the two are merged for an account view). Postgres achieved this with a shared SEQUENCE; in
/// SQLite we draw the next id from the dedicated `seq_tx_tr` table and assign it explicitly.
/// The seq draw and the insert must run on the same connection for the id to be consistent.
pub async fn insert_on(
    conn: &mut SqliteConnection,
    new: &NewTransactionData,
) -> Result<Transaction, AppError> {
    let id: i64 = sqlx::query_scalar("INSERT INTO seq_tx_tr DEFAULT VALUES RETURNING id")
        .fetch_one(&mut *conn)
        .await?;

    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "INSERT INTO transactions (id, value, description, date, account, category) \
         VALUES (?, ?, ?, ?, ?, ?) RETURNING {COLUMNS}"
    ))
    .bind(id)
    .bind(new.value)
    .bind(new.description.as_str())
    .bind(new.date)
    .bind(new.account)
    .bind(new.category)
    .fetch_one(&mut *conn)
    .await?;

    Ok(transaction)
}

pub async fn get_all_of_account_joined(
    pool: &SqlitePool,
    account_id: i32,
) -> Result<Vec<TxJoinRow>, AppError> {
    let rows = sqlx::query_as::<_, TxJoinRow>(&format!(
        "{JOIN_SELECT} WHERE t.account = ? ORDER BY t.date DESC"
    ))
    .bind(account_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_joined(pool: &SqlitePool, id: i32) -> Result<TxJoinRow, AppError> {
    let row = sqlx::query_as::<_, TxJoinRow>(&format!("{JOIN_SELECT} WHERE t.id = ?"))
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

pub async fn update(
    pool: &SqlitePool,
    id: i32,
    new: &NewTransactionData,
) -> Result<Transaction, AppError> {
    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "UPDATE transactions SET value = ?, description = ?, date = ?, account = ?, category = ? \
         WHERE id = ? RETURNING {COLUMNS}"
    ))
    .bind(new.value)
    .bind(new.description.as_str())
    .bind(new.date)
    .bind(new.account)
    .bind(new.category)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}

pub async fn delete(pool: &SqlitePool, id: i32) -> Result<Transaction, AppError> {
    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "DELETE FROM transactions WHERE id = ? RETURNING {COLUMNS}"
    ))
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}
