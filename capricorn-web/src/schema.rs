table! {
    applications (id) {
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

table! {
    jbxx_index (id) {
        id -> Int4,
        empi -> Varchar,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Text,
        avatar -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(applications -> users (provider));

allow_tables_to_appear_in_same_query!(
    applications,
    jbxx_index,
    users,
);
