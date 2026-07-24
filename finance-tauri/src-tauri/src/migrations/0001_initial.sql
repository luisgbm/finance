-- Migration 0001: initial SQLite schema for the Finance desktop app.
--
-- Translated from the original Postgres migration:
--   * SERIAL / sequence-backed PKs        -> INTEGER PRIMARY KEY AUTOINCREMENT
--   * CREATE TYPE ... ENUM                 -> plain TEXT columns (values are the enum's
--                                             snake_case names, e.g. 'expense', 'transfer_income')
--   * BOOLEAN                              -> INTEGER (0 / 1)
--   * TIMESTAMP                            -> TEXT (sqlx stores chrono NaiveDateTime as text)
--   * pgcrypto extension                   -> removed
--
-- Single-user desktop build: this is a fully local, single-user app with no accounts and no
-- authentication, so there is no `app_users` table and no `user_id` column anywhere — every
-- row simply belongs to the one person using the machine. (The web/Postgres backend keeps its
-- multi-user schema; this divergence is intentional and scoped to the Tauri POC.)
--
-- Statements use IF NOT EXISTS so re-running the migration is a no-op. Foreign keys are enabled
-- per-connection via SqliteConnectOptions::foreign_keys(true), which makes the ON DELETE CASCADE
-- clauses behave like the Postgres original.

CREATE TABLE IF NOT EXISTS categories
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    categorytype TEXT NOT NULL,
    name         TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS accounts
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

-- Shared, monotonically increasing id source for transactions AND transfers, replicating
-- the Postgres `transactions_transfers_id_seq` sequence. Both tables draw ids from here so
-- their id spaces never overlap (the React UI merges the two lists and keys rows by id).
CREATE TABLE IF NOT EXISTS seq_tx_tr
(
    id INTEGER PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE IF NOT EXISTS transactions
(
    id          INTEGER PRIMARY KEY,
    value       INTEGER NOT NULL,
    description TEXT    NOT NULL,
    date        TEXT    NOT NULL,
    account     INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    category    INTEGER REFERENCES categories (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS transfers
(
    id                  INTEGER PRIMARY KEY,
    origin_account      INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    destination_account INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    value               INTEGER NOT NULL,
    description         TEXT    NOT NULL,
    date                TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS scheduled_transactions
(
    id                     INTEGER PRIMARY KEY AUTOINCREMENT,
    kind                   TEXT    NOT NULL,
    value                  INTEGER NOT NULL,
    description            TEXT,
    created_date           TEXT    NOT NULL,
    account_id             INTEGER REFERENCES accounts (id) ON DELETE CASCADE,
    category_id            INTEGER REFERENCES categories (id) ON DELETE CASCADE,
    origin_account_id      INTEGER REFERENCES accounts (id) ON DELETE CASCADE,
    destination_account_id INTEGER REFERENCES accounts (id) ON DELETE CASCADE,
    repeat                 INTEGER NOT NULL,
    repeat_freq            TEXT,
    repeat_interval        INTEGER,
    infinite_repeat        INTEGER,
    end_after_repeats      INTEGER,
    current_repeat_count   INTEGER,
    next_date              TEXT
);
