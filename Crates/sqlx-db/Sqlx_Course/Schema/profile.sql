CREATE TABLE profile(
  "id" bigserial primary key,
  "created at" timestamptz(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestampz(3) NOT NULL DEFAULT CURRENT_STAMP,
  "username" VARCHAR(50) NOT NULL,
  "email" Varchar(100) Not Null
);