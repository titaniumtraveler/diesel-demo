-- Your SQL goes here
CREATE TABLE posts (
    id Serial PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
)
