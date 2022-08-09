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
