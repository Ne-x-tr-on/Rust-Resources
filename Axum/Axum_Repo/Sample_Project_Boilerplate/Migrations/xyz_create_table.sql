CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY,
  email text UNIQUE NOT NULL,
  password_hash text NOT NULL,
  name text,
  created_at timestamptz NOT NULL
);

CREATE TABLE IF NOT EXISTS students (
  id uuid PRIMARY KEY,
  name text NOT NULL,
  class text NOT NULL,
  admission_number text UNIQUE NOT NULL,
  created_at timestamptz NOT NULL
);
