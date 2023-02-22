// @generated automatically by Diesel CLI.

diesel::table! {
    movies (title) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
