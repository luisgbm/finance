use diesel::PgConnection;

use crate::database::models::Category;

pub fn get_all_categories(user_id: i32, connection: &PgConnection) -> Vec<Category> {
    crate::database::categories::get_all_categories(user_id, connection)
}