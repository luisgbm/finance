use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth_guard::Authentication;
use crate::db_accounts::DatabaseAccounts;
use crate::models_db::{Account, NewAccount};
use crate::models_routes::{GetAccount, PatchAccount, PostAccount};
use crate::utils;

#[post("/accounts", format = "json", data = "<account>")]
pub fn post_account(account: Json<PostAccount>, auth: Authentication) -> Json<Account> {
    let new_account = NewAccount {
        name: account.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    Json(DatabaseAccounts::new().new_account(&new_account))
}

#[get("/accounts")]
pub fn get_accounts(auth: Authentication) -> Json<Vec<GetAccount>> {
    let accounts = DatabaseAccounts::new().get_all_accounts(auth.token.claims.user_id);

    let mut accounts_with_balance = Vec::new();

    for account in &accounts {
        let balance = utils::get_account_balance(account.id, auth.token.claims.user_id);

        accounts_with_balance.push(GetAccount {
            id: account.id,
            name: account.name.clone(),
            balance,
            user_id: auth.token.claims.user_id,
        });
    }

    Json(accounts_with_balance)
}

#[get("/accounts/<id>")]
pub fn get_account_with_id(id: i32, auth: Authentication) -> Result<Json<GetAccount>, Status> {
    match DatabaseAccounts::new().get_account(id, auth.token.claims.user_id) {
        Ok(account) => {
            let balance = utils::get_account_balance(account.id, auth.token.claims.user_id);

            Ok(Json(GetAccount {
                id: account.id,
                name: account.name.clone(),
                balance,
                user_id: auth.token.claims.user_id,
            }))
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/accounts/<id>", format = "json", data = "<account>")]
pub fn patch_account(id: i32, account: Json<PatchAccount>, auth: Authentication) -> Result<Json<Account>, Status> {
    let account = account.into_inner();

    let account = NewAccount {
        name: account.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    match DatabaseAccounts::new().update_account(id, &account, auth.token.claims.user_id) {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/accounts/<id>")]
pub fn delete_account(id: i32, auth: Authentication) -> Result<Json<Account>, Status> {
    match DatabaseAccounts::new().delete_account(id, auth.token.claims.user_id) {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::NotFound)
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![
        post_account,
        get_accounts,
        get_account_with_id,
        patch_account,
        delete_account
    ]
}