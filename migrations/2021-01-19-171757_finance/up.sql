CREATE EXTENSION pgcrypto;

CREATE TABLE app_users
(
    id       SERIAL PRIMARY KEY NOT NULL,
    name     TEXT               NOT NULL UNIQUE,
    password TEXT               NOT NULL
);

CREATE TYPE category_types AS ENUM ('expense', 'income');

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

CREATE TABLE transactions
(
    id          SERIAL PRIMARY KEY                                   NOT NULL,
    value       INTEGER                                              NOT NULL,
    description TEXT                                                 NOT NULL,
    date        TIMESTAMP                                            NOT NULL,
    account     INTEGER REFERENCES accounts (id) ON DELETE CASCADE   NOT NULL,
    category    INTEGER REFERENCES categories (id) ON DELETE CASCADE NOT NULL,
    user_id     INTEGER REFERENCES app_users (id) ON DELETE CASCADE  NOT NULL
);