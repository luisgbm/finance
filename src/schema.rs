table! {
    use diesel::sql_types::{Integer, Text};

    accounts (id) {
        id -> Integer,
        name -> Text,
        user_id -> Integer,
    }
}

table! {
    use crate::models::CategoryTypesMapping;
    use diesel::sql_types::{Integer, Text};

    categories (id) {
        id -> Integer,
        categorytype -> CategoryTypesMapping,
        name -> Text,
        user_id -> Integer,
    }
}

table! {
    use diesel::sql_types::{Integer, Text, Timestamp};

    transactions (id) {
        id -> Integer,
        value -> Integer,
        description -> Text,
        date -> Timestamp,
        account -> Integer,
        category -> Integer,
        user_id -> Integer,
    }
}

table! {
    use diesel::sql_types::{Integer, Text};

    app_users (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
    }
}

table! {
    use diesel::sql_types::{Integer, Text, Timestamp};

    transfers (id) {
        id -> Integer,
        origin_account -> Integer,
        destination_account -> Integer,
        value -> Integer,
        description -> Text,
        date -> Timestamp,
        user_id -> Integer,
    }
}

joinable!(transactions -> accounts (account));
joinable!(transactions -> categories (category));
joinable!(transactions -> app_users (user_id));
joinable!(accounts -> app_users (user_id));
joinable!(categories -> app_users (user_id));
joinable!(transfers -> app_users (user_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    transactions,
    app_users,
    transfers
);
