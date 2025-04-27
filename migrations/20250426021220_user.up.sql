-- Add up migration script here
CREATE TABLE users(
  card_id TEXT PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE access_logs(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  card_id TEXT NOT NULL,
  accessed_at TEXT NOT NULL,
  FOREIGN KEY (card_id) REFERENCES users(card_id)
);
