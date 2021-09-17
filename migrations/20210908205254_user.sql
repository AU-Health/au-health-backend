-- Add migration script here

-- Create User Table
CREATE TABLE user_account (
    id uuid NOT NULL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);