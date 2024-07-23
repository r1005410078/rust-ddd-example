-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  phone VARCHAR(255) NOT NULL,
  address VARCHAR(255) NOT NULL
);

CREATE INDEX index_users_on_email ON users (email);

