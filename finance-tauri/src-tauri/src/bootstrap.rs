use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous,
};
use sqlx::SqlitePool;

/// Ordered list of schema migrations, embedded into the binary so no external files are
/// needed at runtime. The array index + 1 is the schema version a migration brings the
/// database to; `PRAGMA user_version` records the highest one applied, so each migration
/// runs exactly once and only newer ones run on an existing database.
///
/// Append-only: never edit or reorder an existing entry once it has shipped — add a new one.
const MIGRATIONS: &[&str] = &[
    include_str!("migrations/0001_initial.sql"),
    include_str!("migrations/0002_sessions.sql"),
];

/// Open (creating if needed) the SQLite database at `db_path` and bring its schema up to
/// date, returning a ready-to-use connection pool.
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

    migrate(&pool)
        .await
        .context("failed to migrate the database schema")?;

    Ok(pool)
}

/// Apply any migrations newer than the database's current `user_version`, each inside its own
/// transaction so a failure leaves the version untouched and the database consistent.
async fn migrate(pool: &SqlitePool) -> anyhow::Result<()> {
    let current: i64 = sqlx::query_scalar("PRAGMA user_version")
        .fetch_one(pool)
        .await
        .context("failed to read PRAGMA user_version")?;
    let current = current.max(0) as usize;

    for (index, script) in MIGRATIONS.iter().enumerate().skip(current) {
        let target = index + 1;
        let mut tx = pool.begin().await?;

        // Execute the whole migration script through SQLite's own parser via `raw_sql`, which
        // correctly handles multiple statements, comments and string literals. (A naive
        // `split(';')` would mis-split on a semicolon appearing inside a comment or string.)
        sqlx::raw_sql(script)
            .execute(&mut *tx)
            .await
            .with_context(|| format!("migration {target} failed"))?;

        // `PRAGMA user_version` takes no bind parameters; the value is our own trusted
        // integer, so formatting it into the statement is safe. It is transactional in
        // SQLite, so it commits together with the migration above.
        sqlx::query(&format!("PRAGMA user_version = {target}"))
            .execute(&mut *tx)
            .await
            .with_context(|| format!("failed to record schema version {target}"))?;

        tx.commit().await?;
    }

    Ok(())
}
