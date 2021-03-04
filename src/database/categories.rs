use diesel::prelude::*;
use diesel::result::Error;

use crate::database::models::{Category, CategoryTypes, NewCategory};

pub fn new_category(new_category: &NewCategory, connection: &PgConnection) -> Category {
    use crate::database::schema::categories;

    diesel::insert_into(categories::table)
        .values(new_category)
        .get_result(connection)
        .expect("Error saving new category")
}

pub fn get_all_categories(app_user_id: i32, connection: &PgConnection) -> Vec<Category> {
    use crate::database::schema::categories::dsl::*;

    categories
        .filter(user_id.eq(app_user_id))
        .load::<Category>(connection)
        .expect("Error loading categories")
}

pub fn get_all_categories_by_type(category_type: CategoryTypes, app_user_id: i32, connection: &PgConnection) -> Vec<Category> {
    use crate::database::schema::categories::dsl::*;

    categories
        .filter(user_id.eq(app_user_id))
        .filter(categorytype.eq(category_type))
        .load::<Category>(connection)
        .expect("Error loading categories by type")
}

pub fn get_category(find_id: i32, app_user_id: i32, connection: &PgConnection) -> Result<Category, Error> {
    use crate::database::schema::categories::dsl::*;

    categories
        .filter(user_id.eq(app_user_id))
        .find(find_id)
        .first::<Category>(connection)
}

pub fn update_category(update_id: i32, update_category: &NewCategory, app_user_id: i32, connection: &PgConnection) -> Result<Category, Error> {
    use crate::database::schema::categories::dsl::*;

    diesel::update(categories.filter(user_id.eq(app_user_id)).find(update_id))
        .set((name.eq(update_category.name), categorytype.eq(update_category.categorytype)))
        .get_result::<Category>(connection)
}

pub fn delete_category(delete_id: i32, app_user_id: i32, connection: &PgConnection) -> Result<Category, Error> {
    use crate::database::schema::categories::dsl::*;

    diesel::delete(categories.filter(user_id.eq(app_user_id)).find(delete_id))
        .get_result::<Category>(connection)
}