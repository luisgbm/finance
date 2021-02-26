use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::database::categories::DatabaseCategories;
use crate::database::models::{Category, CategoryTypes, NewCategory};
use crate::routes::auth_guard::Authentication;
use crate::routes::models::{PatchCategory, PostCategory};

#[post("/categories", format = "json", data = "<category>")]
pub fn post_category(category: Json<PostCategory>, auth: Authentication) -> Json<Category> {
    let new_category = NewCategory {
        categorytype: category.categorytype,
        name: category.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    Json(DatabaseCategories::new().new_category(&new_category))
}

#[get("/categories")]
pub fn get_categories(auth: Authentication) -> Json<Vec<Category>> {
    Json(DatabaseCategories::new().get_all_categories(auth.token.claims.user_id))
}

#[get("/categories/<id>")]
pub fn get_category_with_id(id: i32, auth: Authentication) -> Result<Json<Category>, Status> {
    match DatabaseCategories::new().get_category(id, auth.token.claims.user_id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/categories/expense")]
pub fn get_expense_categories(auth: Authentication) -> Json<Vec<Category>> {
    Json(DatabaseCategories::new().get_all_categories_by_type(CategoryTypes::Expense, auth.token.claims.user_id))
}

#[get("/categories/income")]
pub fn get_income_categories(auth: Authentication) -> Json<Vec<Category>> {
    Json(DatabaseCategories::new().get_all_categories_by_type(CategoryTypes::Income, auth.token.claims.user_id))
}

#[patch("/categories/<id>", format = "json", data = "<category>")]
pub fn patch_category(id: i32, category: Json<PatchCategory>, auth: Authentication) -> Result<Json<Category>, Status> {
    let category = NewCategory {
        categorytype: category.categorytype,
        name: category.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    match DatabaseCategories::new().update_category(id, &category, auth.token.claims.user_id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/categories/<id>")]
pub fn delete_category(id: i32, auth: Authentication) -> Result<Json<Category>, Status> {
    match DatabaseCategories::new().delete_category(id, auth.token.claims.user_id) {
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