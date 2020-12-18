use mongodb::sync::{Client, Database};
use mongodb::bson::doc;

use crate::category::Category;

pub struct FinanceDB {
    database: Database
}

impl FinanceDB {
    pub fn new() -> FinanceDB {
        let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();

        FinanceDB {
            database: client.database("financedb")
        }
    }

    pub fn new_category(&self, category: &Category) -> String {
        let category_doc = doc! {
            "category_type": category.category_type.as_str(),
            "name": &category.name
        };

        let object_id = self.database.collection("categories").insert_one(category_doc, None).unwrap().inserted_id;

        object_id.as_object_id().unwrap().to_hex()
    }
}