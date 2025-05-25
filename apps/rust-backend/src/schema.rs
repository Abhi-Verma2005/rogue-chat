// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}
