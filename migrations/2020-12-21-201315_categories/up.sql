CREATE TABLE categories(
    id           SERIAL PRIMARY KEY                   NOT NULL,
    categorytype INTEGER REFERENCES categorytypes(id) NOT NULL,
    name         TEXT                                 NOT NULL
);