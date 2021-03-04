use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::database::models::{Category, CategoryTypes, NewCategory};
use crate::routes::auth_guard::Authentication;
use crate::routes::db_pool::FinancePgDatabase;
use crate::routes::models::{PatchCategory, PostCategory};

#[post("/categories", format = "json", data = "<category>")]
pub fn post_category(category: Json<PostCategory>, auth: Authentication, connection: FinancePgDatabase) -> Json<Category> {
    let new_category = NewCategory {
        categorytype: category.categorytype,
        name: category.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    Json(crate::database::categories::new_category(&new_category, &*connection))
}

#[get("/categories")]
pub fn get_categories(auth: Authentication, connection: FinancePgDatabase) -> Json<Vec<Category>> {
    Json(crate::database::categories::get_all_categories(auth.token.claims.user_id, &*connection))
}

#[get("/categories/<id>")]
pub fn get_category_with_id(id: i32, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Category>, Status> {
    match crate::database::categories::get_category(id, auth.token.claims.user_id, &*connection) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/categories/expense")]
pub fn get_expense_categories(auth: Authentication, connection: FinancePgDatabase) -> Json<Vec<Category>> {
    Json(crate::database::categories::get_all_categories_by_type(CategoryTypes::Expense, auth.token.claims.user_id, &*connection))
}

#[get("/categories/income")]
pub fn get_income_categories(auth: Authentication, connection: FinancePgDatabase) -> Json<Vec<Category>> {
    Json(crate::database::categories::get_all_categories_by_type(CategoryTypes::Income, auth.token.claims.user_id, &*connection))
}

#[patch("/categories/<id>", format = "json", data = "<category>")]
pub fn patch_category(id: i32, category: Json<PatchCategory>, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Category>, Status> {
    let category = NewCategory {
        categorytype: category.categorytype,
        name: category.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    match crate::database::categories::update_category(id, &category, auth.token.claims.user_id, &*connection) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/categories/<id>")]
pub fn delete_category(id: i32, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Category>, Status> {
    match crate::database::categories::delete_category(id, auth.token.claims.user_id, &*connection) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![
        post_category,
        get_categories,
        get_expense_categories,
        get_income_categories,
        get_category_with_id,
        patch_category,
        delete_category
    ]
}