-- Add migration script here
Drop table posts;
Drop table users;

CREATE TYPE UserPermission AS ENUM ('admin', 'user', 'moderator');

CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       username TEXT NOT NULL UNIQUE,
                       email TEXT NOT NULL UNIQUE,
                       password_hash TEXT NOT NULL,
                       UserPermission UserPermission NOT NULL DEFAULT 'user',
                       created_at TIMESTAMP DEFAULT NOW()
);