use diesel::result::{DatabaseErrorKind, Error};
use rocket;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::finance_db::FinanceDB;
use crate::jwt;
use crate::models::{Account, AccountWithBalance, Category,
                    CategoryTypes, NewAccount, NewCategory,
                    NewTransaction, NewUser, Transaction, TransactionJoined,
                    TransactionNoAccount, User};
use crate::utils;

#[post("/login", format = "json", data = "<user>")]
pub fn login(user: Json<NewUser>) -> Result<String, Status> {
    let result = FinanceDB::new().authenticate_user(&user);

    match result {
        Ok(user) => {
            Ok(jwt::create_jwt(&user.name))
        },
        Err(_) => Err(Status::Unauthorized)
    }
}

#[post("/users", format = "json", data = "<user>")]
pub fn post_user(user: Json<NewUser>) -> Result<Json<User>, Status> {
    let result = FinanceDB::new().new_user(&user);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            match err {
                Error::DatabaseError(error_kind, _) => {
                    match error_kind {
                        DatabaseErrorKind::UniqueViolation => Err(Status::Conflict),
                        _ => Err(Status::InternalServerError)
                    }
                }
                Error::NotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError)
            }
        }
    }
}

#[post("/transactions/account/<account_id>", format = "json", data = "<transaction>")]
pub fn post_transaction(account_id: i32, transaction: Json<TransactionNoAccount>) -> Json<Transaction> {
    let t = NewTransaction {
        value: transaction.value,
        description: transaction.description.as_str(),
        date: transaction.date,
        account: account_id,
        category: transaction.category,
    };

    Json(FinanceDB::new().new_transaction(&t))
}

#[post("/accounts", format = "json", data = "<account>")]
pub fn post_account(account: Json<NewAccount>) -> Json<Account> {
    Json(FinanceDB::new().new_account(&account.into_inner()))
}

#[post("/categories", format = "json", data = "<category>")]
pub fn post_category(category: Json<NewCategory>) -> Json<Category> {
    Json(FinanceDB::new().new_category(&category.into_inner()))
}

#[get("/transactions/account/<account_id>")]
pub fn get_transactions(account_id: i32) -> Json<Vec<TransactionJoined>> {
    let mut transactions = Vec::new();

    let joins = FinanceDB::new().get_all_transactions_of_account_joined(account_id);

    for join in &joins {
        transactions.push(utils::create_transaction_join(join));
    }

    Json(transactions)
}

#[get("/accounts")]
pub fn get_accounts() -> Json<Vec<AccountWithBalance>> {
    let accounts = FinanceDB::new().get_all_accounts();

    let mut accounts_with_balance: Vec<AccountWithBalance> = Vec::new();

    for account in &accounts {
        let balance = utils::get_account_balance(account.id);

        accounts_with_balance.push(AccountWithBalance {
            id: account.id,
            name: account.name.clone(),
            balance,
        });
    }

    Json(accounts_with_balance)
}

#[get("/categories")]
pub fn get_categories() -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories())
}

#[get("/categories/expense")]
pub fn get_expense_categories() -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories_by_type(CategoryTypes::Expense))
}

#[get("/categories/income")]
pub fn get_income_categories() -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories_by_type(CategoryTypes::Income))
}

#[get("/transactions/<id>")]
pub fn get_transaction_with_id(id: i32) -> Result<Json<TransactionJoined>, Status> {
    match FinanceDB::new().get_transaction(id) {
        Ok(join) => Ok(Json(utils::create_transaction_join(&join))),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/accounts/<id>")]
pub fn get_account_with_id(id: i32) -> Result<Json<AccountWithBalance>, Status> {
    match FinanceDB::new().get_account(id) {
        Ok(account) => {
            let balance = utils::get_account_balance(account.id);

            Ok(Json(AccountWithBalance {
                id: account.id,
                name: account.name.clone(),
                balance,
            }))
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/categories/<id>")]
pub fn get_category_with_id(id: i32) -> Result<Json<Category>, Status> {
    match FinanceDB::new().get_category(id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/transactions/<id>", format = "json", data = "<transaction>")]
pub fn patch_transaction(id: i32, transaction: Json<NewTransaction>) -> Result<Json<Transaction>, Status> {
    match FinanceDB::new().update_transaction(id, &transaction.into_inner()) {
        Ok(transaction) => Ok(Json(transaction)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/accounts/<id>", format = "json", data = "<account>")]
pub fn patch_account(id: i32, account: Json<NewAccount>) -> Result<Json<Account>, Status> {
    match FinanceDB::new().update_account(id, &account.into_inner()) {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/categories/<id>", format = "json", data = "<category>")]
pub fn patch_category(id: i32, category: Json<NewCategory>) -> Result<Json<Category>, Status> {
    match FinanceDB::new().update_category(id, &category.into_inner()) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/transactions/<id>")]
pub fn delete_transaction(id: i32) -> Result<Json<Transaction>, Status> {
    match FinanceDB::new().delete_transaction(id) {
        Ok(transaction) => Ok(Json(transaction)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/accounts/<id>")]
pub fn delete_account(id: i32) -> Result<Json<Account>, Status> {
    match FinanceDB::new().delete_account(id) {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/categories/<id>")]
pub fn delete_category(id: i32) -> Result<Json<Category>, Status> {
    match FinanceDB::new().delete_category(id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}