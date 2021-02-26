use diesel::prelude::*;
use diesel::result::Error;

use crate::database::finance::FinanceDB;
use crate::database::models::{Category, CategoryTypes, NewCategory};

pub struct DatabaseCategories {
    connection: FinanceDB
}

impl DatabaseCategories {
    pub fn new() -> DatabaseCategories {
        DatabaseCategories {
            connection: FinanceDB::new()
        }
    }

    pub fn new_category(&self, new_category: &NewCategory) -> Category {
        use crate::database::schema::categories;

        diesel::insert_into(categories::table)
            .values(new_category)
            .get_result(&self.connection.db_connection)
            .expect("Error saving new category")
    }

    pub fn get_all_categories(&self, app_user_id: i32) -> Vec<Category> {
        use crate::database::schema::categories::dsl::*;

        categories
            .filter(user_id.eq(app_user_id))
            .load::<Category>(&self.connection.db_connection)
            .expect("Error loading categories")
    }

    pub fn get_all_categories_by_type(&self, category_type: CategoryTypes, app_user_id: i32) -> Vec<Category> {
        use crate::database::schema::categories::dsl::*;

        categories
            .filter(user_id.eq(app_user_id))
            .filter(categorytype.eq(category_type))
            .load::<Category>(&self.connection.db_connection)
            .expect("Error loading categories by type")
    }

    pub fn get_category(&self, find_id: i32, app_user_id: i32) -> Result<Category, Error> {
        use crate::database::schema::categories::dsl::*;

        categories
            .filter(user_id.eq(app_user_id))
            .find(find_id)
            .first::<Category>(&self.connection.db_connection)
    }

    pub fn update_category(&self, update_id: i32, update_category: &NewCategory, app_user_id: i32) -> Result<Category, Error> {
        use crate::database::schema::categories::dsl::*;

        diesel::update(categories.filter(user_id.eq(app_user_id)).find(update_id))
            .set((name.eq(update_category.name), categorytype.eq(update_category.categorytype)))
            .get_result::<Category>(&self.connection.db_connection)
    }

    pub fn delete_category(&self, delete_id: i32, app_user_id: i32) -> Result<Category, Error> {
        use crate::database::schema::categories::dsl::*;

        diesel::delete(categories.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<Category>(&self.connection.db_connection)
    }
}