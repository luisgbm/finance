use sqlx::PgPool;

use crate::error::AppError;
use crate::models::Account;

const COLUMNS: &str = "id, name, user_id";

pub async fn insert(pool: &PgPool, name: &str, user_id: i32) -> Result<Account, AppError> {
    let account = sqlx::query_as::<_, Account>(&format!(
        "INSERT INTO accounts (name, user_id) VALUES ($1, $2) RETURNING {COLUMNS}"
    ))
    .bind(name)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn get_all(pool: &PgPool, user_id: i32) -> Result<Vec<Account>, AppError> {
    let accounts = sqlx::query_as::<_, Account>(&format!(
        "SELECT {COLUMNS} FROM accounts WHERE user_id = $1 ORDER BY id"
    ))
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(accounts)
}

pub async fn get(pool: &PgPool, id: i32, user_id: i32) -> Result<Account, AppError> {
    let account = sqlx::query_as::<_, Account>(&format!(
        "SELECT {COLUMNS} FROM accounts WHERE user_id = $1 AND id = $2"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    name: &str,
    user_id: i32,
) -> Result<Account, AppError> {
    let account = sqlx::query_as::<_, Account>(&format!(
        "UPDATE accounts SET name = $1 WHERE user_id = $2 AND id = $3 RETURNING {COLUMNS}"
    ))
    .bind(name)
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn delete(pool: &PgPool, id: i32, user_id: i32) -> Result<Account, AppError> {
    let account = sqlx::query_as::<_, Account>(&format!(
        "DELETE FROM accounts WHERE user_id = $1 AND id = $2 RETURNING {COLUMNS}"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

/// Compute an account balance entirely in SQL:
///   + income transactions, - expense transactions, - transfers out, + transfers in.
/// Equivalent to the original Rust-side accumulation in `utils::get_account_balance`.
pub async fn balance(pool: &PgPool, account_id: i32, user_id: i32) -> Result<i32, AppError> {
    let balance: i32 = sqlx::query_scalar(
        "SELECT ( \
            COALESCE((SELECT SUM(CASE \
                    WHEN c.categorytype = 'income' THEN t.value \
                    WHEN c.categorytype = 'expense' THEN -t.value \
                    ELSE 0 END) \
                FROM transactions t JOIN categories c ON c.id = t.category \
                WHERE t.account = $1 AND t.user_id = $2), 0) \
            - COALESCE((SELECT SUM(value) FROM transfers \
                WHERE origin_account = $1 AND user_id = $2), 0) \
            + COALESCE((SELECT SUM(value) FROM transfers \
                WHERE destination_account = $1 AND user_id = $2), 0) \
        )::int4",
    )
    .bind(account_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(balance)
}
