-- Add migration script here
CREATE TABLE message (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  user_id BIGINT NOT NULL,
  body VARCHAR(140),
  likes INT NOT NULL DEFAULT 0,

  CONSTRAINT fk_profile FOREIGN KEY (user_id) REFERENCES profile(id)
);

-- Assuming your profile table already has rows with ids 1–10
INSERT INTO message (user_id, body, likes) VALUES
  (1, 'Hello world, this is Lydia''s first post!', 5),
  (2, 'Stella checking in from SQLx training 🚀', 2),
  (3, 'Cate says: Learning Rust is awesome!', 7),
  (4, 'Braisen loves databases ❤️', 3),
  (5, 'David is building his first API with SQLx.', 10),
  (6, 'Kamau joined the chat!', 1),
  (7, 'Vivian is testing the message table now.', 4),
  (8, 'Faith says hi 👋', 6),
  (9, 'Agnes is writing her first migration.', 0),
  (10, 'Cynthia just liked this project!', 8);
