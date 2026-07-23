use sqlx::SqlitePool;

use crate::config::Config;

/// Shared application state, registered once with `app.manage(...)` during setup and
/// injected into every command via `tauri::State<'_, AppState>`.
///
/// The SQLite pool is internally reference-counted and safe to share across concurrent
/// commands, so no `Mutex` is needed.
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Config,
}
