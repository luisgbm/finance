#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;

pub mod models_routes;
pub mod models_db;
pub mod schema;
pub mod db_finance;
pub mod utils;
pub mod routes_scheduled_transactions;
pub mod routes_categories;
pub mod routes_accounts;
pub mod routes_auth;
pub mod routes_transactions;
pub mod routes_transfers;
pub mod jwt;
pub mod auth_guard;
pub mod db_accounts;
pub mod db_categories;
pub mod db_transactions;
pub mod db_transfers;
pub mod db_auth;
pub mod db_scheduled_transactions;
pub mod db_scheduled_transfers;
pub mod routes_scheduled_transfers;
pub mod controller_accounts;
pub mod controller_auth;
pub mod controller_categories;

fn main() {
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    let mut all_routes = Vec::new();

    all_routes.append(&mut routes_categories::get_routes());
    all_routes.append(&mut routes_accounts::get_routes());
    all_routes.append(&mut routes_auth::get_routes());
    all_routes.append(&mut routes_transactions::get_routes());
    all_routes.append(&mut routes_transfers::get_routes());
    all_routes.append(&mut routes_scheduled_transactions::get_routes());
    all_routes.append(&mut routes_scheduled_transfers::get_routes());

    rocket::ignite().mount("/", all_routes).attach(cors).launch();
}
