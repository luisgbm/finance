#![feature(plugin, proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use crate::routes::db_pool::FinancePgDatabase;

mod utils;
mod database;
mod routes;
mod controllers;

fn main() {
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite().mount("/", routes::get_all_routes()).attach(cors).attach(FinancePgDatabase::fairing()).launch();
}
