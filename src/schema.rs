// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        title -> Text,
        pos -> Int4,
    }
}
