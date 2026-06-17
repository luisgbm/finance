use std::sync::Arc;

use sqlx::PgPool;

use crate::config::Config;

/// Shared application state injected into every handler via `axum::extract::State`.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Arc<Config>,
}
