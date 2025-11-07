-- Add up migration script here

CREATE TABLE IF NOT EXISTS urls (
    id BIGINT PRIMARY KEY DEFAULT (1 + floor( random() * 1000000000 ))::BIGINT,
    longurl TEXT NOT NULL,
    shorturl TEXT UNIQUE
);
