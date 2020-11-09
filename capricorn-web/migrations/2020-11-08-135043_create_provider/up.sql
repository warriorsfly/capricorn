CREATE EXTENSION
IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    username VARCHAR(16) NOT NULL,
    email VARCHAR(254) NOT NULL,
    UNIQUE (username, email),
    password TEXT NOT NULL,
    avatar TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

    SELECT diesel_manage_updated_at('users');
