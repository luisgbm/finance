CREATE TABLE categorytype(
                             id   SERIAL PRIMARY KEY NOT NULL,
                             name TEXT               NOT NULL
);

CREATE TABLE category(
                         id           SERIAL PRIMARY KEY                  NOT NULL,
                         categorytype INTEGER REFERENCES categorytype(id) NOT NULL,
                         name         TEXT                                NOT NULL
);