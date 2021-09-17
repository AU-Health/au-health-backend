-- Add migration script here
CREATE TYPE USER_ROLE AS ENUM ('User', 'Admin');


-- Create User Table
CREATE TABLE user_account (
    id uuid NOT NULL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    role USER_ROLE NOT NULL
);