table! {
    accounts (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    categories (id) {
        id -> Int4,
        categorytype -> Int4,
        name -> Text,
    }
}

table! {
    categorytypes (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        value -> Int4,
        description -> Text,
        date -> Timestamp,
        account -> Int4,
        category -> Int4,
    }
}

joinable!(categories -> categorytypes (categorytype));
joinable!(transactions -> accounts (account));
joinable!(transactions -> categories (category));

allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    categorytypes,
    transactions,
);
