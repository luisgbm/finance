use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::AppUser;

/// bcrypt cost must sit in the 4..=31 range; clamp the configured `bf_rounds` into it.
fn clamp_cost(bf_rounds: i32) -> u32 {
    bf_rounds.clamp(4, 31) as u32
}

/// Insert a new user, hashing the password with bcrypt.
///
/// The original Postgres backend hashed via pgcrypto's `crypt()` / `gen_salt('bf', rounds)`,
/// which produces a standard bcrypt (`$2a$`) hash. Here we hash in Rust with the `bcrypt`
/// crate (producing a `$2b$` hash) using the same cost, keeping the stored format a
/// self-describing modular-crypt string that `bcrypt::verify` can validate.
pub async fn insert(
    pool: &SqlitePool,
    name: &str,
    password: &str,
    bf_rounds: i32,
) -> Result<AppUser, AppError> {
    let hashed = bcrypt::hash(password, clamp_cost(bf_rounds))?;

    let user = sqlx::query_as::<_, AppUser>(
        "INSERT INTO app_users (name, password) \
         VALUES (?, ?) \
         RETURNING id, name, password",
    )
    .bind(name)
    .bind(hashed)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Return the user if the provided plaintext password matches the stored bcrypt hash.
///
/// Unlike the Postgres version (which verified the hash inside the SQL `WHERE` clause via
/// `crypt()`), we fetch the row by name and verify with `bcrypt::verify` in Rust.
pub async fn authenticate(
    pool: &SqlitePool,
    name: &str,
    password: &str,
) -> Result<Option<AppUser>, AppError> {
    let user = sqlx::query_as::<_, AppUser>(
        "SELECT id, name, password FROM app_users WHERE name = ?",
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    match user {
        Some(user) if bcrypt::verify(password, &user.password)? => Ok(Some(user)),
        _ => Ok(None),
    }
}
