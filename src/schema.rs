table! {
    use diesel::sql_types::{Integer, Text};

    accounts (id) {
        id -> Integer,
        name -> Text,
        user_id -> Integer,
    }
}

table! {
    use crate::models_db::CategoryTypesMapping;
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

table! {
    use crate::models_db::RepeatFrequenciesMapping;
    use diesel::sql_types::{Integer, Text, Timestamp, Bool, Nullable};

    scheduled_transactions (id) {
        id -> Integer,
        account_id -> Integer,
        value -> Integer,
        description -> Text,
        category_id -> Integer,
        created_date -> Timestamp,
        repeat -> Bool,
        repeat_freq -> Nullable<RepeatFrequenciesMapping>,
        repeat_interval -> Nullable<Integer>,
        infinite_repeat -> Nullable<Bool>,
        end_after_repeats -> Nullable<Integer>,
        current_repeat_count -> Nullable<Integer>,
        next_date -> Nullable<Timestamp>,
        user_id -> Integer,
    }
}

table! {
    use crate::models_db::RepeatFrequenciesMapping;
    use diesel::sql_types::{Integer, Text, Timestamp, Bool, Nullable};

    scheduled_transfers (id) {
        id -> Integer,
        origin_account_id -> Integer,
        destination_account_id -> Integer,
        value -> Integer,
        description -> Text,
        created_date -> Timestamp,
        repeat -> Bool,
        repeat_freq -> Nullable<RepeatFrequenciesMapping>,
        repeat_interval -> Nullable<Integer>,
        infinite_repeat -> Nullable<Bool>,
        end_after_repeats -> Nullable<Integer>,
        current_repeat_count -> Nullable<Integer>,
        next_date -> Nullable<Timestamp>,
        user_id -> Integer,
    }
}

joinable!(transactions -> accounts (account));
joinable!(transactions -> categories (category));
joinable!(transactions -> app_users (user_id));
joinable!(accounts -> app_users (user_id));
joinable!(categories -> app_users (user_id));
joinable!(transfers -> app_users (user_id));
joinable!(transfers -> accounts (origin_account));
joinable!(scheduled_transactions -> accounts (account_id));
joinable!(scheduled_transactions -> categories (category_id));
joinable!(scheduled_transactions -> app_users (user_id));
joinable!(scheduled_transfers -> accounts (origin_account_id));
joinable!(scheduled_transfers -> app_users (user_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    transactions,
    app_users,
    transfers,
    scheduled_transactions,
    scheduled_transfers
);
