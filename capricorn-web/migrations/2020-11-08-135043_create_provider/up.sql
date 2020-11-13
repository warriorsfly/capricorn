CREATE EXTENSION
IF NOT EXISTS "uuid-ossp";

CREATE TABLE service_providers
(
    id SERIAL PRIMARY KEY,
    name VARCHAR(16) NOT NULL,
    email VARCHAR(254) NOT NULL,
    UNIQUE (name, email),
    password TEXT NOT NULL,
    avatar TEXT NOT NULL DEFAULT '',
    salt TEXT NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

    SELECT diesel_manage_updated_at('service_providers');
