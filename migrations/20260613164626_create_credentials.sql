-- Add migration script here
CREATE TABLE credentials (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    service TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);