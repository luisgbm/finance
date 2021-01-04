CREATE TABLE categories(
    id           SERIAL PRIMARY KEY NOT NULL,
    categorytype category_types     NOT NULL,
    name         TEXT               NOT NULL
);