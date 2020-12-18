CREATE TABLE users
(
    id int primary key,
    name VARCHAR(16) NOT NULL,
    phone VARCHAR(254) NOT NULL,
    UNIQUE (phone),
    password TEXT NOT NULL,
    avatar TEXT NOT NULL DEFAULT '',
    salt TEXT NOT NULL,
    slug TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('users');