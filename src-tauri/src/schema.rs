// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Integer,
        category -> Text,
        title -> Text,
        amount -> Float,
        date -> Timestamp,
    }
}
