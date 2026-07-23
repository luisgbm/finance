use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

/// Application-wide error type. Each variant maps to an HTTP-style status code, preserved
/// from the original REST backend so the React frontend's `err.response.status` checks
/// (401 -> logout, 409 -> "user exists") keep working after the move to Tauri IPC.
///
/// When a command returns `Err(AppError)`, Tauri serializes it (via the `Serialize` impl
/// below) and the JS `invoke(...)` promise rejects with `{ status, message }`.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,
    #[error("bad request")]
    BadRequest,
    #[error("internal server error: {0}")]
    Internal(String),
}

impl AppError {
    /// The HTTP-style status code the frontend inspects. No longer an `axum` `StatusCode`
    /// now that responses travel over IPC rather than HTTP.
    fn status_code(&self) -> u16 {
        match self {
            AppError::Unauthorized => 401,
            AppError::NotFound => 404,
            AppError::Conflict => 409,
            AppError::BadRequest => 400,
            AppError::Internal(_) => 500,
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialization is the single choke point through which every command error passes,
        // so log internal failures here (their detail is not otherwise surfaced to the UI).
        if let AppError::Internal(message) = self {
            log::error!("internal error: {}", message);
        }
        let mut state = serializer.serialize_struct("AppError", 2)?;
        state.serialize_field("status", &self.status_code())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

/// Translate a sqlx error into an `AppError`. A unique-violation becomes `Conflict`
/// (e.g. duplicate user name); a missing row becomes `NotFound`; everything else is `Internal`.
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => AppError::Conflict,
            other => AppError::Internal(other.to_string()),
        }
    }
}

/// Password hashing errors (bcrypt) are always internal failures.
impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::Internal(format!("password hashing error: {err}"))
    }
}
