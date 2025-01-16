// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        useragent -> Text,
        jwt -> Text,
        refresh_token -> Text,
        is_active -> Bool,
        last_activity -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        salt -> Text,
        totp_secret -> Nullable<Text>,
        backup_codes -> Nullable<Jsonb>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
