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

allow_tables_to_appear_in_same_query!(
    jbxx_index,
    users,
);
