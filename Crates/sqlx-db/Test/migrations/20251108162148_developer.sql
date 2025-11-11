-- Add migration script here
-- Up
CREATE TABLE developer (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

-- Down
-- DROP TABLE developer;
