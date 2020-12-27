CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE services
(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- belong to user
    provider int NOT NULL REFERENCES providers(id),
    slug TEXT UNIQUE NOT NULL,
    -- application name
    name TEXT NOT NULL,
    -- application description
    description TEXT NOT NULL,
    UNIQUE (provider, name),
    -- application icon
    icon TEXT NOT NULL DEFAULT '',
    -- application secret
    secret TEXT NOT NULL,
    -- application key
    key TEXT NOT NULL,
    -- is the app enableing now
    enabled boolean NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX services_provider_id_idx ON services (provider);
SELECT diesel_manage_updated_at('services');
