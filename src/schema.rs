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

joinable!(categories -> categorytypes (categorytype));

allow_tables_to_appear_in_same_query!(
    categories,
    categorytypes,
);
