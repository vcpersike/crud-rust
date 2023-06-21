-- Your SQL goes here
CREATE TABLE usuario (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  password TEXT NOT NULL,
  email TEXT NOT NULL,
  phone TEXT NOT NULL
);