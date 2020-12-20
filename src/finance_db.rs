use mongodb::sync::{Client, Database};
use mongodb::bson::{doc, Bson};

use crate::category::{Category, CategoryType};

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

    pub fn get_all_categories(&self) -> Vec<Category> {
        let cursor = self.database.collection("categories").find(None, None).unwrap();

        let mut categories = Vec::new();

        for result in cursor {
            if let Ok(item) = result {
                println!("{:?}", item);
                let mut name = String::new();
                let mut category_type_opt = None;
                let mut object_id = String::new();

                if let Some(&Bson::String(ref n)) = item.get("name") {
                    name = n.to_string();
                }

                if let Some(&Bson::String(ref c)) = item.get("category_type") {
                    category_type_opt = CategoryType::from_str(c);
                }

                if let Some(&Bson::ObjectId(ref id)) = item.get("_id") {
                    object_id = id.to_hex();
                }

                if let Some(category_type) = category_type_opt {
                    categories.push(Category::new_with_id(category_type, name.as_str(), object_id.as_str()));
                }

            }
        }

        categories
    }
}