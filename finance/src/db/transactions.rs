use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::{CategoryTypes, NewTransactionData, Transaction};

const COLUMNS: &str = "id, value, description, date, account, category, user_id";

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
    pub user_id: i32,
}

const JOIN_SELECT: &str = "SELECT t.id, t.value, t.description, t.date, \
    t.category AS category_id, c.categorytype AS category_type, c.name AS category_name, \
    t.account AS account_id, a.name AS account_name, t.user_id \
    FROM transactions t \
    JOIN categories c ON c.id = t.category \
    JOIN accounts a ON a.id = t.account";

pub async fn insert(pool: &PgPool, new: &NewTransactionData) -> Result<Transaction, AppError> {
    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "INSERT INTO transactions (value, description, date, account, category, user_id) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING {COLUMNS}"
    ))
    .bind(new.value)
    .bind(new.description.as_str())
    .bind(new.date)
    .bind(new.account)
    .bind(new.category)
    .bind(new.user_id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}

pub async fn get_all_of_account_joined(
    pool: &PgPool,
    account_id: i32,
    user_id: i32,
) -> Result<Vec<TxJoinRow>, AppError> {
    let rows = sqlx::query_as::<_, TxJoinRow>(&format!(
        "{JOIN_SELECT} WHERE t.user_id = $1 AND t.account = $2 ORDER BY t.date DESC"
    ))
    .bind(user_id)
    .bind(account_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_joined(pool: &PgPool, id: i32, user_id: i32) -> Result<TxJoinRow, AppError> {
    let row = sqlx::query_as::<_, TxJoinRow>(&format!(
        "{JOIN_SELECT} WHERE t.user_id = $1 AND t.id = $2"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    new: &NewTransactionData,
    user_id: i32,
) -> Result<Transaction, AppError> {
    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "UPDATE transactions SET value = $1, description = $2, date = $3, account = $4, category = $5 \
         WHERE user_id = $6 AND id = $7 RETURNING {COLUMNS}"
    ))
    .bind(new.value)
    .bind(new.description.as_str())
    .bind(new.date)
    .bind(new.account)
    .bind(new.category)
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}

pub async fn delete(pool: &PgPool, id: i32, user_id: i32) -> Result<Transaction, AppError> {
    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "DELETE FROM transactions WHERE user_id = $1 AND id = $2 RETURNING {COLUMNS}"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}
