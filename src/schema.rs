table! {
    access_tokens (id) {
        id -> Int4,
        token -> Varchar,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(access_tokens -> users (id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    users,
);
