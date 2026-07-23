use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous,
};
use sqlx::SqlitePool;

/// The SQLite schema, embedded into the binary so no external file is needed at runtime.
const SCHEMA: &str = include_str!("schema.sql");

/// Open (creating if needed) the SQLite database at `db_path` and apply the schema,
/// returning a ready-to-use connection pool.
///
/// Kept separate from the Tauri wiring so integration tests can build a pool against a
/// throwaway database on their own runtime.
pub async fn init(db_path: &Path) -> anyhow::Result<SqlitePool> {
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

    Ok(pool)
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
