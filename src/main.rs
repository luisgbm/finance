#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::finance_db::FinanceDB;
use crate::models::{Account, Category, CategoryTypes, NewAccount, NewCategory, NewTransaction, Transaction, TransactionJoined, TransactionNoAccount};

pub mod models;
pub mod schema;
pub mod finance_db;
pub mod utils;

#[post("/transactions/account/<account_id>", format = "json", data = "<transaction>")]
fn post_transaction(account_id: i32, transaction: Json<TransactionNoAccount>) -> Json<Transaction> {
    let t = NewTransaction {
        value: transaction.value,
        description: transaction.description.as_str(),
        date: transaction.date,
        account: account_id,
        category: transaction.category
    };

    Json(FinanceDB::new().new_transaction(&t))
}

#[post("/accounts", format = "json", data = "<account>")]
fn post_account(account: Json<NewAccount>) -> Json<Account> {
    Json(FinanceDB::new().new_account(&account.into_inner()))
}

#[post("/categories", format = "json", data = "<category>")]
fn post_category(category: Json<NewCategory>) -> Json<Category> {
    Json(FinanceDB::new().new_category(&category.into_inner()))
}

#[get("/transactions/account/<account_id>")]
fn get_transactions(account_id: i32) -> Json<Vec<TransactionJoined>> {
    let mut transactions = Vec::new();

    let joins = FinanceDB::new().get_all_transactions_of_account_joined(account_id);

    for join in &joins {
        transactions.push(utils::create_transaction_join(join));
    }

    Json(transactions)
}

#[get("/accounts")]
fn get_accounts() -> Json<Vec<Account>> {
    Json(FinanceDB::new().get_all_accounts())
}

#[get("/categories")]
fn get_categories() -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories())
}

#[get("/categories/expense")]
fn get_expense_categories() -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories_by_type(CategoryTypes::Expense))
}

#[get("/categories/income")]
fn get_income_categories() -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories_by_type(CategoryTypes::Income))
}

#[get("/transactions/<id>")]
fn get_transaction_with_id(id: i32) -> Result<Json<TransactionJoined>, Status> {
    match FinanceDB::new().get_transaction(id) {
        Ok(join) => Ok(Json(utils::create_transaction_join(&join))),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/accounts/<id>")]
fn get_account_with_id(id: i32) -> Result<Json<Account>, Status> {
    match FinanceDB::new().get_account(id) {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/categories/<id>")]
fn get_category_with_id(id: i32) -> Result<Json<Category>, Status> {
    match FinanceDB::new().get_category(id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/transactions/<id>", format = "json", data = "<transaction>")]
fn patch_transaction(id: i32, transaction: Json<NewTransaction>) -> Result<Json<Transaction>, Status> {
    match FinanceDB::new().update_transaction(id, &transaction.into_inner()) {
        Ok(transaction) => Ok(Json(transaction)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/accounts/<id>", format = "json", data = "<account>")]
fn patch_account(id: i32, account: Json<NewAccount>) -> Result<Json<Account>, Status> {
    match FinanceDB::new().update_account(id, &account.into_inner()) {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/categories/<id>", format = "json", data = "<category>")]
fn patch_category(id: i32, category: Json<NewCategory>) -> Result<Json<Category>, Status> {
    match FinanceDB::new().update_category(id, &category.into_inner()) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/transactions/<id>")]
fn delete_transaction(id: i32) -> Result<Json<Transaction>, Status> {
    match FinanceDB::new().delete_transaction(id) {
        Ok(transaction) => Ok(Json(transaction)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/accounts/<id>")]
fn delete_account(id: i32) -> Result<Json<Account>, Status> {
    match FinanceDB::new().delete_account(id) {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/categories/<id>")]
fn delete_category(id: i32) -> Result<Json<Category>, Status> {
    match FinanceDB::new().delete_category(id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

fn main() {
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite().mount("/", routes![
        post_category,
        get_categories,
        get_expense_categories,
        get_income_categories,
        get_category_with_id,
        patch_category,
        delete_category,
        post_account,
        get_accounts,
        get_account_with_id,
        patch_account,
        delete_account,
        post_transaction,
        get_transactions,
        get_transaction_with_id,
        patch_transaction,
        delete_transaction
    ]).attach(cors).launch();
}
