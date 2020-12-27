table! {
    providers (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Text,
        avatar -> Text,
        salt -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    services (id) {
        id -> Uuid,
        provider -> Int4,
        slug -> Text,
        name -> Text,
        description -> Text,
        icon -> Text,
        secret -> Text,
        key -> Text,
        enabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(services -> providers (provider));

allow_tables_to_appear_in_same_query!(
    providers,
    services,
);
