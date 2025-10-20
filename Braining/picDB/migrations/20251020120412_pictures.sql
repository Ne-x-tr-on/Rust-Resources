-- Add migration script here
CREATE TABLE course.pictures (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    event_type TEXT NOT NULL, -- Wedding, Hike, Funeral, etc.
    file_path TEXT NOT NULL,
    uploaded_at TIMESTAMP DEFAULT NOW()
);
