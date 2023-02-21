// @generated automatically by Diesel CLI.

diesel::table! {
    blog_posts (title) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
