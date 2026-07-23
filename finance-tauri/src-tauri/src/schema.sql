-- SQLite schema for the Finance desktop POC.
--
-- Translated from the original Postgres migration:
--   * SERIAL / sequence-backed PKs        -> INTEGER PRIMARY KEY AUTOINCREMENT
--   * CREATE TYPE ... ENUM                 -> plain TEXT columns (values are the enum's
--                                             snake_case names, e.g. 'expense', 'transfer_income')
--   * BOOLEAN                              -> INTEGER (0 / 1)
--   * TIMESTAMP                            -> TEXT (sqlx stores chrono NaiveDateTime as text)
--   * pgcrypto extension                   -> removed (password hashing happens in Rust)
--
-- Every statement is idempotent (IF NOT EXISTS) so it can run on every launch.
-- Foreign keys are enabled per-connection via SqliteConnectOptions::foreign_keys(true),
-- which makes the ON DELETE CASCADE clauses below behave like the Postgres original.

CREATE TABLE IF NOT EXISTS app_users
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    name     TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS categories
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    categorytype TEXT    NOT NULL,
    name         TEXT    NOT NULL,
    user_id      INTEGER NOT NULL REFERENCES app_users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS accounts
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    TEXT    NOT NULL,
    user_id INTEGER NOT NULL REFERENCES app_users (id) ON DELETE CASCADE
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
    category    INTEGER REFERENCES categories (id) ON DELETE CASCADE,
    user_id     INTEGER NOT NULL REFERENCES app_users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS transfers
(
    id                  INTEGER PRIMARY KEY,
    origin_account      INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    destination_account INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    value               INTEGER NOT NULL,
    description         TEXT    NOT NULL,
    date                TEXT    NOT NULL,
    user_id             INTEGER NOT NULL REFERENCES app_users (id) ON DELETE CASCADE
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
    next_date              TEXT,
    user_id                INTEGER NOT NULL REFERENCES app_users (id) ON DELETE CASCADE
);
