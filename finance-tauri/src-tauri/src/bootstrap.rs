use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use axum::Router;
use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous,
};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::handlers;
use crate::state::AppState;

/// The SQLite schema, embedded into the binary so no external file is needed at runtime.
const SCHEMA: &str = include_str!("schema.sql");

/// Open (creating if needed) the SQLite database at `db_path`, apply the schema, and build
/// the fully-configured Axum router (the complete original REST API with CORS + tracing and
/// the `AppState` already injected). Kept separate from [`start`] so it can be exercised
/// directly by integration tests on their own runtime.
pub async fn build_router(db_path: &Path) -> anyhow::Result<Router> {
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        // Enforce foreign keys so ON DELETE CASCADE behaves like the Postgres original.
        .foreign_keys(true)
        // WAL + NORMAL is the recommended durable-yet-fast setup for a local desktop DB.
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        // Wait instead of erroring if another connection briefly holds the write lock.
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .context("failed to open the SQLite database")?;

    run_schema(&pool)
        .await
        .context("failed to initialise the database schema")?;

    let state = AppState {
        pool,
        config: Arc::new(Config::local()),
    };

    let app = Router::new()
        .merge(handlers::auth::routes())
        .merge(handlers::categories::routes())
        .merge(handlers::accounts::routes())
        .merge(handlers::transactions::routes())
        .merge(handlers::transfers::routes())
        .merge(handlers::scheduled_transactions::routes())
        // Permissive CORS: the webview origin (tauri://localhost / http://tauri.localhost)
        // differs from the API origin (127.0.0.1:<port>), and auth is via bearer token.
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    Ok(app)
}

/// Build the router and start serving it on an OS-assigned loopback port, returning the
/// chosen port so the caller can point the webview at it.
///
/// The server task is spawned onto Tauri's async runtime and keeps running for the life of
/// the process; only the bind + spawn happen before this function returns.
pub async fn start(db_path: &Path) -> anyhow::Result<u16> {
    let app = build_router(db_path).await?;

    // Port 0 => the OS picks a free ephemeral port, so we never collide with anything else.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("failed to bind the local API port")?;
    let port = listener
        .local_addr()
        .context("failed to read the local API port")?
        .port();

    tracing::info!("finance backend listening on http://127.0.0.1:{port}");

    tauri::async_runtime::spawn(async move {
        if let Err(err) = axum::serve(listener, app).await {
            tracing::error!("embedded server stopped with error: {err}");
        }
    });

    Ok(port)
}

/// Execute the embedded schema. Statements are separated by `;`; the schema contains no
/// semicolons inside string literals, so a simple split is safe here.
async fn run_schema(pool: &SqlitePool) -> anyhow::Result<()> {
    for statement in SCHEMA.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        sqlx::query(statement)
            .execute(pool)
            .await
            .with_context(|| format!("failed to execute schema statement: {statement}"))?;
    }
    Ok(())
}
