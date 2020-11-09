CREATE EXTENSION
IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    user_name VARCHAR(16) NOT NULL,
    email VARCHAR(254) NOT NULL,
    UNIQUE (user_name, email),
    password TEXT NOT NULL,
    avatar TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

    SELECT diesel_manage_updated_at('users');
