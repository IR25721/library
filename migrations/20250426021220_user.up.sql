-- Add up migration script here
CREATE TABLE users(
  card_id TEXT PRIMARY KEY,
  name TEXT NOT NULL
);
