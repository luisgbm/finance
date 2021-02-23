CREATE EXTENSION pgcrypto;

CREATE TABLE app_users
(
    id       SERIAL PRIMARY KEY NOT NULL,
    name     TEXT               NOT NULL UNIQUE,
    password TEXT               NOT NULL
);

CREATE TYPE category_types AS ENUM ('expense', 'income', 'transfer_income', 'transfer_expense');

CREATE TABLE categories
(
    id           SERIAL PRIMARY KEY                                  NOT NULL,
    categorytype category_types                                      NOT NULL,
    name         TEXT                                                NOT NULL,
    user_id      INTEGER REFERENCES app_users (id) ON DELETE CASCADE NOT NULL
);

CREATE TABLE accounts
(
    id      SERIAL PRIMARY KEY                                  NOT NULL,
    name    TEXT                                                NOT NULL,
    user_id INTEGER REFERENCES app_users (id) ON DELETE CASCADE NOT NULL
);

CREATE SEQUENCE transactions_transfers_id_seq;

CREATE TABLE transactions
(
    id          INTEGER DEFAULT nextval('transactions_transfers_id_seq') PRIMARY KEY NOT NULL,
    value       INTEGER                                                              NOT NULL,
    description TEXT                                                                 NOT NULL,
    date        TIMESTAMP                                                            NOT NULL,
    account     INTEGER REFERENCES accounts (id) ON DELETE CASCADE                   NOT NULL,
    category    INTEGER REFERENCES categories (id) ON DELETE CASCADE,
    user_id     INTEGER REFERENCES app_users (id) ON DELETE CASCADE                  NOT NULL
);

CREATE TABLE transfers
(
    id                  INTEGER DEFAULT nextval('transactions_transfers_id_seq') PRIMARY KEY NOT NULL,
    origin_account      INTEGER REFERENCES accounts (id) ON DELETE CASCADE                   NOT NULL,
    destination_account INTEGER REFERENCES accounts (id) ON DELETE CASCADE                   NOT NULL,
    value               INTEGER                                                              NOT NULL,
    description         TEXT                                                                 NOT NULL,
    date                TIMESTAMP                                                            NOT NULL,
    user_id             INTEGER REFERENCES app_users (id) ON DELETE CASCADE                  NOT NULL
);

CREATE TYPE repeat_frequencies AS ENUM ('days', 'weeks', 'months', 'years');

CREATE TYPE scheduled_transacion_kind AS ENUM ('transaction', 'transfer');

CREATE TABLE scheduled_transactions
(
    id                     SERIAL PRIMARY KEY                                  NOT NULL,
    kind                   scheduled_transacion_kind                           NOT NULL,
    value                  INTEGER                                             NOT NULL,
    description            TEXT,
    created_date           TIMESTAMP                                           NOT NULL,
    account_id             INTEGER REFERENCES accounts (id) ON DELETE CASCADE,
    category_id            INTEGER REFERENCES categories (id) ON DELETE CASCADE,
    origin_account_id      INTEGER REFERENCES accounts (id) ON DELETE CASCADE,
    destination_account_id INTEGER REFERENCES accounts (id) ON DELETE CASCADE,
    repeat                 BOOLEAN                                             NOT NULL,
    repeat_freq            repeat_frequencies,
    repeat_interval        INTEGER,
    infinite_repeat        BOOLEAN,
    end_after_repeats      INTEGER,
    current_repeat_count   INTEGER,
    next_date              TIMESTAMP,
    user_id                INTEGER REFERENCES app_users (id) ON DELETE CASCADE NOT NULL
);