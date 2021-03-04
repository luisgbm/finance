use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::controllers;
use crate::database::models::{Account, NewAccount};
use crate::routes::auth_guard::Authentication;
use crate::routes::db_pool::FinancePgDatabase;
use crate::routes::models::{GetAccount, PatchAccount, PostAccount};
use crate::utils;

#[post("/accounts", format = "json", data = "<account>")]
pub fn post_account(account: Json<PostAccount>, auth: Authentication, connection: FinancePgDatabase) -> Json<GetAccount> {
    let new_account = NewAccount {
        name: account.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    let account = crate::database::accounts::new_account(&new_account, &*connection);

    Json(GetAccount {
        id: account.id,
        name: account.name,
        balance: 0,
        user_id: account.user_id,
    })
}

#[get("/accounts")]
pub fn get_accounts(auth: Authentication, connection: FinancePgDatabase) -> Json<Vec<GetAccount>> {
    Json(controllers::accounts::get_all_accounts(auth.token.claims.user_id, &*connection))
}

#[get("/accounts/<id>")]
pub fn get_account_with_id(id: i32, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<GetAccount>, Status> {
    if let Some(account) = controllers::accounts::get_account(id, auth.token.claims.user_id, &*connection) {
        return Ok(Json(account));
    }

    Err(Status::NotFound)
}

#[patch("/accounts/<id>", format = "json", data = "<account>")]
pub fn patch_account(id: i32, account: Json<PatchAccount>, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<GetAccount>, Status> {
    let account = account.into_inner();

    let account = NewAccount {
        name: account.name.as_str(),
        user_id: auth.token.claims.user_id,
    };

    match crate::database::accounts::update_account(id, &account, auth.token.claims.user_id, &*connection) {
        Ok(account) => {
            Ok(Json(GetAccount {
                id: account.id,
                name: account.name,
                balance: utils::get_account_balance(account.id, account.user_id, &*connection),
                user_id: account.user_id,
            }))
        },
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/accounts/<id>")]
pub fn delete_account(id: i32, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Account>, Status> {
    match crate::database::accounts::delete_account(id, auth.token.claims.user_id, &*connection) {
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