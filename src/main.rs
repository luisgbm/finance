#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

extern crate dotenv;

use rocket_contrib::json::Json;
use rocket::http::Status;

pub mod models;
pub mod schema;
pub mod finance_db;

use crate::finance_db::FinanceDB;
use crate::models::{CategoryType, NewCategoryType, NewCategory, Category};

#[post("/categories", format = "json", data = "<category>")]
fn post_category(category: Json<NewCategory>) -> Json<Category> {
    Json(FinanceDB::new().new_category(&category.into_inner()))
}

#[post("/categorytypes", format = "json", data = "<category_type>")]
fn post_category_type(category_type: Json<NewCategoryType>) -> Json<CategoryType> {
    Json(FinanceDB::new().new_category_type(&category_type.into_inner()))
}

#[get("/categories")]
fn get_categories() -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories())
}

#[get("/categorytypes")]
fn get_category_types() -> Json<Vec<CategoryType>> {
    Json(FinanceDB::new().get_all_category_types())
}

#[get("/categories/<id>")]
fn get_category_with_id(id: i32) -> Result<Json<Category>, Status> {
    match FinanceDB::new().get_category(id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/categorytypes/<id>")]
fn get_category_type_with_id(id: i32) -> Result<Json<CategoryType>, Status> {
    match FinanceDB::new().get_category_type(id) {
        Ok(category_type) => Ok(Json(category_type)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/categorytypes/<id>", format = "json", data = "<category_type>")]
fn patch_category_type(id: i32, category_type: Json<NewCategoryType>) -> Result<Json<CategoryType>, Status> {
    match FinanceDB::new().update_category_type(id, &category_type.into_inner()) {
        Ok(category_type) => Ok(Json(category_type)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/categorytypes/<id>")]
fn delete_category_type(id: i32) -> Result<Json<CategoryType>, Status> {
    match FinanceDB::new().delete_category_type(id) {
        Ok(category_type) => Ok(Json(category_type)),
        Err(_) => Err(Status::NotFound)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![
        post_category_type,
        get_category_types,
        get_category_type_with_id,
        patch_category_type,
        delete_category_type,
        post_category,
        get_categories,
        get_category_with_id
    ]).launch();
}
