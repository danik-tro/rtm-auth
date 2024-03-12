-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL,
    email VARCHAR(255) CONSTRAINT email_unique_cnt UNIQUE NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
);
