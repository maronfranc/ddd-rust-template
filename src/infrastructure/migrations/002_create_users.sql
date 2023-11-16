CREATE TABLE IF NOT EXISTS users (
    id          SERIAL PRIMARY KEY,
    email       VARCHAR NOT NULL CONSTRAINT email_unique_constraint UNIQUE,
    username    VARCHAR NOT NULL CONSTRAINT username_unique_constraint UNIQUE,
    password    VARCHAR NOT NULL
        constraint password_min_length CHECK (LENGTH(password) >= 6),
    id_person   INT,
    FOREIGN KEY (id_person) REFERENCES persons(id)
);
