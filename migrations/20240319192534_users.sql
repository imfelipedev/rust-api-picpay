CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    identification VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    balance INT DEFAULT 0 NOT NULL,
    user_type VARCHAR NOT NULL
);