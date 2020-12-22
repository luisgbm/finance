CREATE TABLE transactions(
    id          SERIAL PRIMARY KEY                NOT NULL,
    value       INTEGER                           NOT NULL,
    description TEXT                              NOT NULL,
    date        TIMESTAMP                         NOT NULL,
    account     INTEGER REFERENCES accounts(id)   NOT NULL,
    category    INTEGER REFERENCES categories(id) NOT NULL
);