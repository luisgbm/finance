use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// Application-wide error type. Each variant maps to an HTTP status code, mirroring the
/// status codes returned by the original backend.
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
    fn status(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        if let AppError::Internal(ref message) = self {
            log::error!("internal error: {}", message);
        }
        (status, Json(json!({ "error": self.to_string() }))).into_response()
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
