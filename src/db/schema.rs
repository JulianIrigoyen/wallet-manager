// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Int4,
        #[max_length = 255]
        wallet_address -> Varchar,
        #[max_length = 10]
        transaction_type -> Varchar,
        amount -> Int8,
    }
}
