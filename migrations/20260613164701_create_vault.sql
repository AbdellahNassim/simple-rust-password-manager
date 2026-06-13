-- Add migration script here
CREATE TABLE vault (
    id INTEGER PRIMARY KEY,
    password_hash TEXT NOT NULL,
    salt TEXT NOT NULL
);