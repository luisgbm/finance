use crate::database::categories::DatabaseCategories;
use crate::database::models::Category;

pub fn get_all_categories(user_id: i32) -> Vec<Category> {
    DatabaseCategories::new().get_all_categories(user_id)
}