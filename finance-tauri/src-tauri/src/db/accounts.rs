use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::Account;

const COLUMNS: &str = "id, name";

pub async fn insert(pool: &SqlitePool, name: &str) -> Result<Account, AppError> {
    let account = sqlx::query_as::<_, Account>(&format!(
        "INSERT INTO accounts (name) VALUES (?) RETURNING {COLUMNS}"
    ))
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Account>, AppError> {
    let accounts = sqlx::query_as::<_, Account>(&format!(
        "SELECT {COLUMNS} FROM accounts ORDER BY id"
    ))
    .fetch_all(pool)
    .await?;

    Ok(accounts)
}

pub async fn get(pool: &SqlitePool, id: i32) -> Result<Account, AppError> {
    let account = sqlx::query_as::<_, Account>(&format!(
        "SELECT {COLUMNS} FROM accounts WHERE id = ?"
    ))
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn update(pool: &SqlitePool, id: i32, name: &str) -> Result<Account, AppError> {
    let account = sqlx::query_as::<_, Account>(&format!(
        "UPDATE accounts SET name = ? WHERE id = ? RETURNING {COLUMNS}"
    ))
    .bind(name)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn delete(pool: &SqlitePool, id: i32) -> Result<Account, AppError> {
    let account = sqlx::query_as::<_, Account>(&format!(
        "DELETE FROM accounts WHERE id = ? RETURNING {COLUMNS}"
    ))
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

/// Compute an account balance:
///   + income transactions, - expense transactions, - transfers out, + transfers in.
///
/// The Postgres version did this in a single query that reused the `$1`/`$2` placeholders
/// several times. SQLite uses positional `?` placeholders, so instead of binding the same
/// values many times we compute the three component sums separately and combine them in
/// Rust (equivalent to the original `utils::get_account_balance` accumulation).
pub async fn balance(pool: &SqlitePool, account_id: i32) -> Result<i32, AppError> {
    // Income (+) minus expense (-) across this account's transactions.
    let transactions_sum: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(CASE \
                WHEN c.categorytype = 'income' THEN t.value \
                WHEN c.categorytype = 'expense' THEN -t.value \
                ELSE 0 END), 0) \
            FROM transactions t JOIN categories c ON c.id = t.category \
            WHERE t.account = ?",
    )
    .bind(account_id)
    .fetch_one(pool)
    .await?;

    // Transfers leaving this account (subtracted).
    let transfers_out: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(value), 0) FROM transfers WHERE origin_account = ?",
    )
    .bind(account_id)
    .fetch_one(pool)
    .await?;

    // Transfers entering this account (added).
    let transfers_in: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(value), 0) FROM transfers WHERE destination_account = ?",
    )
    .bind(account_id)
    .fetch_one(pool)
    .await?;

    Ok((transactions_sum - transfers_out + transfers_in) as i32)
}
