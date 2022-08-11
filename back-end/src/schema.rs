table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        img -> Text,
        body -> Text,
        summary -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
        password -> Text,
        is_staff -> Bool,
        is_active -> Bool,
        is_superuser -> Bool,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
