use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::database::accounts::DatabaseAccounts;
use crate::database::models::{NewTransfer, Transfer};
use crate::database::transfers::DatabaseTransfers;
use crate::routes::auth_guard::Authentication;
use crate::routes::models::{PatchTransfer, PostTransfer};

#[post("/transfers/from/<origin_account>/to/<destination_account>", format = "json", data = "<new_transfer>")]
pub fn post_transfer(origin_account: i32, destination_account: i32, new_transfer: Json<PostTransfer>, auth: Authentication) -> Result<Json<Transfer>, Status> {
    match DatabaseAccounts::new().get_account(origin_account, auth.token.claims.user_id) {
        Ok(_) => {
            match DatabaseAccounts::new().get_account(destination_account, auth.token.claims.user_id) {
                Ok(_) => {
                    let transfer: PostTransfer = new_transfer.into_inner();

                    let transfer = NewTransfer {
                        origin_account,
                        destination_account,
                        value: transfer.value,
                        description: transfer.description.as_str(),
                        date: transfer.date,
                        user_id: auth.token.claims.user_id,
                    };

                    Ok(Json(DatabaseTransfers::new().new_transfer(&transfer)))
                }
                Err(_) => Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/transfers/<id>")]
pub fn get_transfer_with_id(id: i32, auth: Authentication) -> Result<Json<Transfer>, Status> {
    match DatabaseTransfers::new().get_transfer(id, auth.token.claims.user_id) {
        Ok(transfer) => Ok(Json(transfer)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/transfers/<id>", format = "json", data = "<transfer>")]
pub fn patch_transfer(id: i32, transfer: Json<PatchTransfer>, auth: Authentication) -> Result<Json<Transfer>, Status> {
    match DatabaseAccounts::new().get_account(transfer.origin_account, auth.token.claims.user_id) {
        Ok(_) => {
            match DatabaseAccounts::new().get_account(transfer.destination_account, auth.token.claims.user_id) {
                Ok(_) => {
                    let transfer = NewTransfer {
                        origin_account: transfer.origin_account,
                        destination_account: transfer.destination_account,
                        value: transfer.value,
                        description: transfer.description.as_str(),
                        date: transfer.date,
                        user_id: auth.token.claims.user_id,
                    };

                    match DatabaseTransfers::new().update_transfer(id, &transfer, auth.token.claims.user_id) {
                        Ok(obj) => Ok(Json(obj)),
                        Err(_) => Err(Status::NotFound)
                    }
                }
                Err(_) => Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/transfers/<id>")]
pub fn delete_transfer(id: i32, auth: Authentication) -> Result<Json<Transfer>, Status> {
    match DatabaseTransfers::new().delete_transfer(id, auth.token.claims.user_id) {
        Ok(transfer) => Ok(Json(transfer)),
        Err(_) => Err(Status::NotFound)
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![
        post_transfer,
        get_transfer_with_id,
        patch_transfer,
        delete_transfer
    ]
}