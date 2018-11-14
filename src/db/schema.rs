#![allow(proc_macro_derive_resolution_fallback)]

table! {
    groups (id) {
        id -> Integer,
        display_name -> Varchar,
        created_on -> Timestamp,
        modified_on -> Timestamp,
    }
}

table! {
    group_users (id) {
        id -> Integer,
        user_id -> Integer,
        group_id -> Integer,
        created_on -> Timestamp,
        modified_on -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        password_hash -> Varchar,
        display_name -> Varchar,
        created_on -> Timestamp,
        modified_on -> Timestamp,
    }
}

table! {
    user_roles (id) {
        id -> Integer,
        user_id -> Integer,
        role_id -> Integer,
        target -> Integer,
        created_on -> Timestamp,
        modified_on -> Timestamp,
    }
}

joinable!(group_users -> groups (group_id));
joinable!(group_users -> users (user_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    groups,
    group_users,
    roles,
    users,
    user_roles,
);
