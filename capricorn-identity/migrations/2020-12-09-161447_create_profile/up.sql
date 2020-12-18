CREATE TABLE user_profiles
(
    id int primary key,
    user_id int,
    phone VARCHAR(254) NOT NULL,
    UNIQUE (phone),
    avatar TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('user_profiles');