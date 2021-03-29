table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        hash -> Varchar,
        create_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    confirmations,
    users,
);
