use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::database::models::{NewTransfer, Transfer};
use crate::routes::auth_guard::Authentication;
use crate::routes::db_pool::FinancePgDatabase;
use crate::routes::models::{PatchTransfer, PostTransfer};

#[post("/api/transfers/from/<origin_account>/to/<destination_account>", format = "json", data = "<new_transfer>")]
pub fn post_transfer(origin_account: i32, destination_account: i32, new_transfer: Json<PostTransfer>, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Transfer>, Status> {
    match crate::database::accounts::get_account(origin_account, auth.token.claims.user_id, &*connection) {
        Ok(_) => {
            match crate::database::accounts::get_account(destination_account, auth.token.claims.user_id, &*connection) {
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

                    Ok(Json(crate::database::transfers::new_transfer(&transfer, &*connection)))
                }
                Err(_) => Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/api/transfers/<id>")]
pub fn get_transfer_with_id(id: i32, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Transfer>, Status> {
    match crate::database::transfers::get_transfer(id, auth.token.claims.user_id, &*connection) {
        Ok(transfer) => Ok(Json(transfer)),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/api/transfers/<id>", format = "json", data = "<transfer>")]
pub fn patch_transfer(id: i32, transfer: Json<PatchTransfer>, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Transfer>, Status> {
    match crate::database::accounts::get_account(transfer.origin_account, auth.token.claims.user_id, &*connection) {
        Ok(_) => {
            match crate::database::accounts::get_account(transfer.destination_account, auth.token.claims.user_id, &*connection) {
                Ok(_) => {
                    let transfer = NewTransfer {
                        origin_account: transfer.origin_account,
                        destination_account: transfer.destination_account,
                        value: transfer.value,
                        description: transfer.description.as_str(),
                        date: transfer.date,
                        user_id: auth.token.claims.user_id,
                    };

                    match crate::database::transfers::update_transfer(id, &transfer, auth.token.claims.user_id, &*connection) {
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

#[delete("/api/transfers/<id>")]
pub fn delete_transfer(id: i32, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Transfer>, Status> {
    match crate::database::transfers::delete_transfer(id, auth.token.claims.user_id, &*connection) {
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