CREATE EXTENSION pgcrypto;

CREATE TABLE users
(
    id       SERIAL PRIMARY KEY NOT NULL,
    name     TEXT               NOT NULL UNIQUE,
    password TEXT               NOT NULL
);