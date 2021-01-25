#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;

pub mod models;
pub mod schema;
pub mod finance_db;
pub mod utils;
pub mod routes;
pub mod jwt;

fn main() {
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite().mount("/", routes![
        routes::post_category,
        routes::get_categories,
        routes::get_expense_categories,
        routes::get_income_categories,
        routes::get_category_with_id,
        routes::patch_category,
        routes::delete_category,
        routes::post_account,
        routes::get_accounts,
        routes::get_account_with_id,
        routes::patch_account,
        routes::delete_account,
        routes::post_transaction,
        routes::get_transactions,
        routes::get_transaction_with_id,
        routes::patch_transaction,
        routes::delete_transaction,
        routes::post_user,
        routes::login,
        routes::validate_token,
        routes::post_transfer,
        routes::get_transfer_with_id,
        routes::patch_transfer,
        routes::delete_transfer
    ]).attach(cors).launch();
}
