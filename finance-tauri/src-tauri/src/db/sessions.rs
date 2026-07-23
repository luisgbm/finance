use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::AppError;

/// Create a new session for `user_id` and return its opaque token.
///
/// The token is a random UUID v4 (122 bits of entropy) — unguessable, so possession of a
/// valid token is what authenticates a caller. It is persisted so a login survives an app
/// restart without re-entering the password (and without storing the password anywhere).
pub async fn create(pool: &SqlitePool, user_id: i32) -> Result<String, AppError> {
    let token = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO sessions (token, user_id, created_date) VALUES (?, ?, ?)")
        .bind(&token)
        .bind(user_id)
        .bind(Utc::now().naive_utc())
        .execute(pool)
        .await?;

    Ok(token)
}

/// Resolve a session token to its user id, or [`AppError::Unauthorized`] if the token is
/// unknown (never issued, or invalidated by logout / user deletion). The 401 drives the
/// frontend to clear the stored token and return to the login screen.
pub async fn resolve(pool: &SqlitePool, token: &str) -> Result<i32, AppError> {
    let user_id: Option<i32> = sqlx::query_scalar("SELECT user_id FROM sessions WHERE token = ?")
        .bind(token)
        .fetch_optional(pool)
        .await?;

    user_id.ok_or(AppError::Unauthorized)
}

/// Delete a session (logout). Removing an unknown token is a no-op, so this is safe to call
/// best-effort from the frontend without worrying about the exact state.
pub async fn delete(pool: &SqlitePool, token: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sessions WHERE token = ?")
        .bind(token)
        .execute(pool)
        .await?;

    Ok(())
}
