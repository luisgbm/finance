use diesel::result::{DatabaseErrorKind, Error};
use jsonwebtoken::TokenData;
use rocket;
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request};
use rocket_contrib::json::Json;

use crate::finance_db::FinanceDB;
use crate::jwt;
use crate::jwt::Claims;
use crate::models::{Account, AccountNoUser, AccountWithBalance, Category, CategoryNoUser, CategoryTypes, NewAccount, NewAppUser, NewCategory, NewTransaction, Transaction, TransactionJoined, TransactionNoAccount, TransactionNoUser};
use crate::utils;

#[derive(Debug)]
pub struct Authentication {
    token: TokenData<Claims>
}

#[derive(Debug)]
pub enum AuthenticationError {
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for Authentication {
    type Error = AuthenticationError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let min_header_len = "Bearer ".len();

        let auth_header = request.headers().get_one("Authorization");

        match auth_header {
            None => Failure((Status::Unauthorized, AuthenticationError::Missing)),
            Some(auth_header) => {
                if auth_header.len() < min_header_len {
                    return Failure((Status::Unauthorized, AuthenticationError::Invalid));
                }

                match jwt::validate_jwt(&auth_header[min_header_len..].to_string()) {
                    Ok(token) => {
                        Success(Authentication {
                            token
                        })
                    }
                    Err(_) => Failure((Status::Unauthorized, AuthenticationError::Invalid))
                }
            }
        }
    }
}

fn login_internal(user: &NewAppUser) -> Result<String, Status> {
    let result = FinanceDB::new().authenticate_user(user);

    match result {
        Ok(user) => {
            Ok(jwt::create_jwt(user.id))
        },
        Err(_) => Err(Status::Unauthorized)
    }
}

#[post("/login", format = "json", data = "<user>")]
pub fn login(user: Json<NewAppUser>) -> Result<String, Status> {
    login_internal(&user.into_inner())
}

#[post("/users", format = "json", data = "<user_json>")]
pub fn post_user(user_json: Json<NewAppUser>) -> Result<String, Status> {
    let result = FinanceDB::new().new_user(&user_json);

    match result {
        Ok(user) => {
            let new_app_user = NewAppUser {
                name: user.name.as_str(),
                password: user_json.password,
            };

            login_internal(&new_app_user)
        },
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
pub fn post_transaction(account_id: i32, transaction: Json<TransactionNoAccount>, auth: Authentication) -> Result<Json<Transaction>, Status> {
    let account = FinanceDB::new().get_account(account_id, auth.token.claims.user_id);

    match account {
        Ok(_) => {
            let t = NewTransaction {
                value: transaction.value,
                description: transaction.description.as_str(),
                date: transaction.date,
                account: account_id,
                category: transaction.category,
                user_id: auth.token.claims.user_id,
            };

            return Ok(Json(FinanceDB::new().new_transaction(&t)))
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[post("/accounts", format = "json", data = "<account>")]
pub fn post_account(account: Json<AccountNoUser>, auth: Authentication) -> Json<Account> {
    let new_account = NewAccount {
        name: account.into_inner().name,
        user_id: auth.token.claims.user_id,
    };

    Json(FinanceDB::new().new_account(&new_account))
}

#[post("/categories", format = "json", data = "<category>")]
pub fn post_category(category: Json<CategoryNoUser>, auth: Authentication) -> Json<Category> {
    let new_category = category.into_inner();

    let new_category = NewCategory {
        categorytype: new_category.categorytype,
        name: new_category.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    Json(FinanceDB::new().new_category(&new_category))
}

#[get("/transactions/account/<account_id>")]
pub fn get_transactions(account_id: i32, auth: Authentication) -> Result<Json<Vec<TransactionJoined>>, Status> {
    let account = FinanceDB::new().get_account(account_id, auth.token.claims.user_id);

    match account {
        Ok(_) => {
            let mut transactions = Vec::new();

            let joins = FinanceDB::new().get_all_transactions_of_account_joined(account_id, auth.token.claims.user_id);

            for join in &joins {
                transactions.push(utils::create_transaction_join(join, auth.token.claims.user_id));
            }

            return Ok(Json(transactions))
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/accounts")]
pub fn get_accounts(auth: Authentication) -> Json<Vec<AccountWithBalance>> {
    let accounts = FinanceDB::new().get_all_accounts(auth.token.claims.user_id);

    let mut accounts_with_balance: Vec<AccountWithBalance> = Vec::new();

    for account in &accounts {
        let balance = utils::get_account_balance(account.id, auth.token.claims.user_id);

        accounts_with_balance.push(AccountWithBalance {
            id: account.id,
            name: account.name.clone(),
            balance,
            user_id: auth.token.claims.user_id
        });
    }

    Json(accounts_with_balance)
}

#[get("/categories")]
pub fn get_categories(auth: Authentication) -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories(auth.token.claims.user_id))
}

#[get("/categories/expense")]
pub fn get_expense_categories(auth: Authentication) -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories_by_type(CategoryTypes::Expense, auth.token.claims.user_id))
}

#[get("/categories/income")]
pub fn get_income_categories(auth: Authentication) -> Json<Vec<Category>> {
    Json(FinanceDB::new().get_all_categories_by_type(CategoryTypes::Income, auth.token.claims.user_id))
}

#[get("/transactions/<id>")]
pub fn get_transaction_with_id(id: i32, auth: Authentication) -> Result<Json<TransactionJoined>, Status> {
    match FinanceDB::new().get_transaction(id, auth.token.claims.user_id) {
        Ok(join) => Ok(Json(utils::create_transaction_join(&join, auth.token.claims.user_id))),
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/accounts/<id>")]
pub fn get_account_with_id(id: i32, auth: Authentication) -> Result<Json<AccountWithBalance>, Status> {
    match FinanceDB::new().get_account(id, auth.token.claims.user_id) {
        Ok(account) => {
            let balance = utils::get_account_balance(account.id, auth.token.claims.user_id);

            Ok(Json(AccountWithBalance {
                id: account.id,
                name: account.name.clone(),
                balance,
                user_id: auth.token.claims.user_id,
            }))
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/categories/<id>")]
pub fn get_category_with_id(id: i32, auth: Authentication) -> Result<Json<Category>, Status> {
    match FinanceDB::new().get_category(id, auth.token.claims.user_id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/transactions/<id>", format = "json", data = "<transaction>")]
pub fn patch_transaction(id: i32, transaction: Json<TransactionNoUser>, auth: Authentication) -> Result<Json<Transaction>, Status> {
    let transaction = transaction.into_inner();

    let transaction = NewTransaction {
        value: transaction.value,
        description: transaction.description.clone(),
        date: transaction.date.clone(),
        account: transaction.account,
        category: transaction.category,
        user_id: auth.token.claims.user_id,
    };

    match FinanceDB::new().update_transaction(id, &transaction, auth.token.claims.user_id) {
        Ok(transaction) => Ok(Json(transaction)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/accounts/<id>", format = "json", data = "<account>")]
pub fn patch_account(id: i32, account: Json<AccountNoUser>, auth: Authentication) -> Result<Json<Account>, Status> {
    let account = account.into_inner();

    let account = NewAccount {
        name: account.name,
        user_id: auth.token.claims.user_id,
    };

    match FinanceDB::new().update_account(id, &account, auth.token.claims.user_id) {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/categories/<id>", format = "json", data = "<category>")]
pub fn patch_category(id: i32, category: Json<CategoryNoUser>, auth: Authentication) -> Result<Json<Category>, Status> {
    let category = category.into_inner();

    let category = NewCategory {
        categorytype: category.categorytype,
        name: category.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    match FinanceDB::new().update_category(id, &category, auth.token.claims.user_id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/transactions/<id>")]
pub fn delete_transaction(id: i32, auth: Authentication) -> Result<Json<Transaction>, Status> {
    match FinanceDB::new().delete_transaction(id, auth.token.claims.user_id) {
        Ok(transaction) => Ok(Json(transaction)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/accounts/<id>")]
pub fn delete_account(id: i32, auth: Authentication) -> Result<Json<Account>, Status> {
    match FinanceDB::new().delete_account(id, auth.token.claims.user_id) {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/categories/<id>")]
pub fn delete_category(id: i32, auth: Authentication) -> Result<Json<Category>, Status> {
    match FinanceDB::new().delete_category(id, auth.token.claims.user_id) {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(Status::NotFound)
    }
}