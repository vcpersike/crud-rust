// @generated automatically by Diesel CLI.

diesel::table! {
    usuario (id) {
        id -> Int4,
        name -> Text,
        password -> Text,
        email -> Text,
        phone -> Text,
    }
}
