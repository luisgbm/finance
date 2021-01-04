table! {
    use diesel::sql_types::{Integer, Text};

    accounts (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    use crate::models::CategoryTypesMapping;
    use diesel::sql_types::{Integer, Text};

    categories (id) {
        id -> Integer,
        categorytype -> CategoryTypesMapping,
        name -> Text,
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
    }
}

joinable!(transactions -> accounts (account));
joinable!(transactions -> categories (category));

allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    transactions,
);
