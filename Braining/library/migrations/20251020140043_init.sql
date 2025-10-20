-- Add migration script here
-- Create the schema
CREATE SCHEMA library;

-- Create LMS tables inside library schema
CREATE TABLE library.books (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    published_at DATE
);

CREATE TABLE library.members (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL
);

CREATE TABLE library.loans (
    id UUID PRIMARY KEY,
    book_id UUID REFERENCES library.books(id),
    member_id UUID REFERENCES library.members(id),
    loaned_at TIMESTAMP DEFAULT NOW(),
    returned_at TIMESTAMP
);
