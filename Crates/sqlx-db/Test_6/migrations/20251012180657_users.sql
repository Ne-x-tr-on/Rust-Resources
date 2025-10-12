-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL,
    gender VARCHAR(10) CHECK (gender IN ('Male', 'Female', 'Other')),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Optional: create index on email for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
