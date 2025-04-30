-- Add migration script here
CREATE TABLE IF NOT EXISTS res_icon (
    id TEXT PRIMARY KEY,
    tid TEXT NOT NULL,
    BLOB TEXT NOT NULL
);