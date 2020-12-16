#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use crate::category::{Category, CategoryType};
use crate::transaction::Transaction;
use crate::account::Account;

use chrono::prelude::*;
use serde_json::json;

mod category;
mod transaction;
mod account;

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
    rocket::ignite().mount("/", routes![index]).launch();
}
