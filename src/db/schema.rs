#![allow(proc_macro_derive_resolution_fallback)]

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        display_name -> Varchar,
        created_on -> Timestamp,
        modified_on -> Timestamp,
    }
}
