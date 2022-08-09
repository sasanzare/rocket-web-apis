table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        image -> Text,
        body -> Text,
        summary -> Text,
        published -> Bool,
        published_date -> Timestamp,
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
        created_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
