use sqlx::PgPool;

use crate::error::AppError;
use crate::models::AppUser;

/// Insert a new user, hashing the password in the database with pgcrypto's
/// `crypt()` / `gen_salt('bf', rounds)` (blowfish), preserving the original scheme so
/// that users created by the previous backend continue to authenticate.
pub async fn insert(
    pool: &PgPool,
    name: &str,
    password: &str,
    bf_rounds: i32,
) -> Result<AppUser, AppError> {
    let user = sqlx::query_as::<_, AppUser>(
        "INSERT INTO app_users (name, password) \
         VALUES ($1, crypt($2, gen_salt('bf', $3))) \
         RETURNING id, name, password",
    )
    .bind(name)
    .bind(password)
    .bind(bf_rounds)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Return the user if the provided plaintext password matches the stored bcrypt hash.
pub async fn authenticate(
    pool: &PgPool,
    name: &str,
    password: &str,
) -> Result<Option<AppUser>, AppError> {
    let user = sqlx::query_as::<_, AppUser>(
        "SELECT id, name, password FROM app_users \
         WHERE name = $1 AND password = crypt($2, password)",
    )
    .bind(name)
    .bind(password)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
