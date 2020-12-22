use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::result::Error;

use crate::models::{CategoryType, NewCategoryType, NewCategory, Category};

pub struct FinanceDB {
    connection: PgConnection
}

impl FinanceDB {
    pub fn new() -> FinanceDB {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        FinanceDB {
            connection
        }
    }

    pub fn new_category(&self, new_category: &NewCategory) -> Category {
        use crate::schema::categories;

        diesel::insert_into(categories::table)
            .values(new_category)
            .get_result(&self.connection)
            .expect("Error saving new category")
    }

    pub fn new_category_type(&self, new_category_type: &NewCategoryType) -> CategoryType {
        use crate::schema::categorytypes;

        diesel::insert_into(categorytypes::table)
            .values(new_category_type)
            .get_result(&self.connection)
            .expect("Error saving new category type")
    }

    pub fn get_all_categories(&self) -> Vec<Category> {
        use crate::schema::categories::dsl::*;

        categories
            .load::<Category>(&self.connection)
            .expect("Error loading categories")
    }

    pub fn get_all_category_types(&self) -> Vec<CategoryType> {
        use crate::schema::categorytypes::dsl::*;

        categorytypes
            .load::<CategoryType>(&self.connection)
            .expect("Error loading category types")
    }

    pub fn get_category(&self, find_id: i32) -> Result<Category, Error> {
        use crate::schema::categories::dsl::*;

        categories
            .find(find_id)
            .first::<Category>(&self.connection)
    }

    pub fn get_category_type(&self, find_id: i32) -> Result<CategoryType, Error> {
        use crate::schema::categorytypes::dsl::*;

        categorytypes
            .find(find_id)
            .first::<CategoryType>(&self.connection)
    }

    pub fn update_category(&self, update_id: i32, update_category: &NewCategory) -> Result<Category, Error> {
        use crate::schema::categories::dsl::*;

        diesel::update(categories.find(update_id))
            .set(name.eq(update_category.name))
            .get_result::<Category>(&self.connection)
    }

    pub fn update_category_type(&self, update_id: i32, update_category_type: &NewCategoryType) -> Result<CategoryType, Error> {
        use crate::schema::categorytypes::dsl::*;

        diesel::update(categorytypes.find(update_id))
            .set(name.eq(update_category_type.name))
            .get_result::<CategoryType>(&self.connection)
    }

    pub fn delete_category(&self, delete_id: i32) -> Result<Category, Error> {
        use crate::schema::categories::dsl::*;

        diesel::delete(categories.find(delete_id))
            .get_result::<Category>(&self.connection)
    }

    pub fn delete_category_type(&self, delete_id: i32) -> Result<CategoryType, Error> {
        use crate::schema::categorytypes::dsl::*;

        diesel::delete(categorytypes.find(delete_id))
            .get_result::<CategoryType>(&self.connection)
    }
}