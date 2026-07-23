use std::sync::Arc;

use sqlx::SqlitePool;

use crate::config::Config;

/// Shared application state injected into every handler via `axum::extract::State`.
/// Identical in shape to the original Postgres backend, except the pool is a SQLite pool.
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Arc<Config>,
}
