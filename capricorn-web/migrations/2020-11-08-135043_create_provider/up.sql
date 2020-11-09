CREATE EXTENSION
IF NOT EXISTS "uuid-ossp";

CREATE TABLE providers
(
    id SERIAL PRIMARY KEY,
    name VARCHAR(16) NOT NULL,
    email VARCHAR(254) NOT NULL,
    UNIQUE (name, email),
    password TEXT NOT NULL,
    avatar TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

    SELECT diesel_manage_updated_at('providers');
