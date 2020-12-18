#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use crate::category::{Category, CategoryType};
use crate::transaction::Transaction;
use crate::finance_db::FinanceDB;
use crate::account::Account;

use chrono::prelude::*;
use serde_json::json;
use rocket_contrib::json::Json;

mod category;
mod transaction;
mod account;
mod finance_db;

#[post("/category", format = "json", data = "<category>")]
fn post_category(category: Json<Category>) -> String {
    FinanceDB::new().new_category(&category.into_inner())
}

#[get("/")]
fn index() -> String {
    let market = Category::new(CategoryType::Expense, "Market");
    let salary = Category::new(CategoryType::Income, "Salary");

    let t1 = Transaction::new(
        &market,
        10,
        Utc.ymd(2020, 12, 16).and_hms(10, 15, 00)
    );

    let t2 = Transaction::new(
        &salary,
        30,
        Utc.ymd(2020, 12, 16).and_hms(10, 16, 00)
    );

    let mut bradesco = Account::new("Bradesco");

    bradesco.add_transaction(&t1);
    bradesco.add_transaction(&t2);

    json!(&bradesco).to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![index, post_category]).launch();
}
