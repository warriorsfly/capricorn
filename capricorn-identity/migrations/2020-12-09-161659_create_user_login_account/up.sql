CREATE TABLE user_accounts
(

    user_id int,
    -- 0手机号 1微信
    identifier_type int NOT NULL,
    identifier VARCHAR(64) NOT NULL,
    primary key(identifier_type,identifier),
    cryptographic VARCHAR(16),
    user_id int NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('user_accounts');