use crate::db_categories::DatabaseCategories;
use crate::models_db::Category;

pub fn get_all_categories(user_id: i32) -> Vec<Category> {
    DatabaseCategories::new().get_all_categories(user_id)
}