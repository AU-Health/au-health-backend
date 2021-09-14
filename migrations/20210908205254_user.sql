-- Add migration script here

-- Create User Table
CREATE TABLE users(
    id uuid NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);