use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth_guard::Authentication;
use crate::db_accounts::DatabaseAccounts;
use crate::db_scheduled_transfers::DatabaseScheduledTransfers;
use crate::db_transfers::DatabaseTransfers;
use crate::models_db::{NewScheduledTransfer, NewTransfer, ScheduledTransfer, Transfer};
use crate::models_routes::{GetScheduledTransfer, PatchScheduledTransfer, PostScheduledTransfer, PostScheduledTransferPay};
use crate::utils;

#[post("/scheduled-transfers/<id>/pay", format = "json", data = "<scheduled_transfer>")]
pub fn post_scheduled_transfer_pay(id: i32, scheduled_transfer: Json<PostScheduledTransferPay>, auth: Authentication) -> Result<Json<Transfer>, Status> {
    match DatabaseScheduledTransfers::new().get_scheduled_transfer(id, auth.token.claims.user_id) {
        Ok(scheduled_transfer_tuple) => {
            if let Err(_) = DatabaseAccounts::new().get_account(scheduled_transfer.origin_account_id, auth.token.claims.user_id) {
                return Err(Status::NotFound);
            }

            if let Err(_) = DatabaseAccounts::new().get_account(scheduled_transfer.destination_account_id, auth.token.claims.user_id) {
                return Err(Status::NotFound);
            }

            let new_transfer = NewTransfer {
                origin_account: scheduled_transfer.origin_account_id,
                destination_account: scheduled_transfer.destination_account_id,
                value: scheduled_transfer.value,
                description: scheduled_transfer.description.as_str(),
                date: scheduled_transfer.date.clone(),
                user_id: auth.token.claims.user_id,
            };

            let new_transfer = DatabaseTransfers::new().new_transfer(&new_transfer);

            let scheduled_transfer = scheduled_transfer_tuple.0;

            if scheduled_transfer.repeat == false {
                DatabaseScheduledTransfers::new().delete_scheduled_transfer(id, auth.token.claims.user_id).unwrap();
                return Ok(Json(new_transfer));
            } else {
                let new_repeat_count = scheduled_transfer.current_repeat_count.unwrap() + 1;

                if scheduled_transfer.infinite_repeat.unwrap() == false && new_repeat_count >= scheduled_transfer.end_after_repeats.unwrap() {
                    DatabaseScheduledTransfers::new().delete_scheduled_transfer(id, auth.token.claims.user_id).unwrap();
                    return Ok(Json(new_transfer));
                }

                let new_date = utils::calculate_next_date(
                    scheduled_transfer.created_date,
                    scheduled_transfer.repeat,
                    scheduled_transfer.repeat_freq.unwrap(),
                    scheduled_transfer.repeat_interval.unwrap(),
                    new_repeat_count,
                );

                let scheduled_transfer_paid = NewScheduledTransfer {
                    origin_account_id: scheduled_transfer.origin_account_id,
                    destination_account_id: scheduled_transfer.destination_account_id,
                    value: scheduled_transfer.value,
                    description: scheduled_transfer.description.as_str(),
                    created_date: scheduled_transfer.created_date.clone(),
                    repeat: scheduled_transfer.repeat,
                    repeat_freq: scheduled_transfer.repeat_freq,
                    repeat_interval: scheduled_transfer.repeat_interval,
                    infinite_repeat: scheduled_transfer.infinite_repeat,
                    end_after_repeats: scheduled_transfer.end_after_repeats,
                    current_repeat_count: Some(new_repeat_count),
                    next_date: Some(new_date),
                    user_id: auth.token.claims.user_id,
                };

                match DatabaseScheduledTransfers::new().update_scheduled_transfer(id, &scheduled_transfer_paid, auth.token.claims.user_id) {
                    Ok(_) => Ok(Json(new_transfer)),
                    Err(_) => Err(Status::InternalServerError)
                }
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

fn internal_get_new_scheduled_transfer_from_post_patch<'a>(scheduled_transfer: &'a Json<PostScheduledTransfer>, auth: &'a Authentication) -> Option<NewScheduledTransfer<'a>> {
    let repeat = scheduled_transfer.repeat;
    let mut repeat_freq = None;
    let mut repeat_interval = None;
    let mut infinite_repeat = None;
    let mut end_after_repeats = None;
    let mut current_repeat_count = None;
    let next_date = Some(scheduled_transfer.created_date.clone());

    if repeat == true {
        match scheduled_transfer.repeat_freq {
            Some(value) => {
                repeat_freq = Some(value);
            }
            None => {
                return None;
            }
        }

        match scheduled_transfer.repeat_interval {
            Some(value) => {
                repeat_interval = Some(value);
            }
            None => {
                return None;
            }
        }

        match scheduled_transfer.infinite_repeat {
            Some(value) => {
                infinite_repeat = Some(value);

                if value == true {
                    end_after_repeats = None;
                } else {
                    match scheduled_transfer.end_after_repeats {
                        Some(value) => {
                            end_after_repeats = Some(value);
                        }
                        None => {
                            return None;
                        }
                    }
                }
            }
            None => {
                return None;
            }
        }

        current_repeat_count = Some(0);
    }

    Some(NewScheduledTransfer {
        origin_account_id: scheduled_transfer.origin_account_id,
        destination_account_id: scheduled_transfer.destination_account_id,
        value: scheduled_transfer.value,
        description: scheduled_transfer.description.as_str(),
        created_date: scheduled_transfer.created_date.clone(),
        repeat: scheduled_transfer.repeat,
        repeat_freq,
        repeat_interval,
        infinite_repeat,
        end_after_repeats,
        current_repeat_count,
        next_date,
        user_id: auth.token.claims.user_id,
    })
}

#[post("/scheduled-transfers", format = "json", data = "<scheduled_transfer>")]
pub fn post_scheduled_transfer(scheduled_transfer: Json<PostScheduledTransfer>, auth: Authentication) -> Result<Json<ScheduledTransfer>, Status> {
    match DatabaseAccounts::new().get_account(scheduled_transfer.origin_account_id, auth.token.claims.user_id) {
        Ok(_) => {
            match DatabaseAccounts::new().get_account(scheduled_transfer.destination_account_id, auth.token.claims.user_id) {
                Ok(_) => {
                    match internal_get_new_scheduled_transfer_from_post_patch(&scheduled_transfer, &auth) {
                        Some(new_scheduled_transfer) => {
                            return Ok(Json(DatabaseScheduledTransfers::new().new_scheduled_transfer(&new_scheduled_transfer)));
                        }
                        None => Err(Status::BadRequest)
                    }
                }
                Err(_) => Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/scheduled-transfers/<id>")]
pub fn get_scheduled_transfer_with_id(id: i32, auth: Authentication) -> Result<Json<GetScheduledTransfer>, Status> {
    match DatabaseScheduledTransfers::new().get_scheduled_transfer(id, auth.token.claims.user_id) {
        Ok(tuple) => {
            match utils::create_scheduled_transfer_join(&tuple) {
                Ok(get_scheduled_transfer) => {
                    Ok(Json(get_scheduled_transfer))
                }
                Err(_) => Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/scheduled-transfers/<id>", format = "json", data = "<scheduled_transfer_patch>")]
pub fn patch_scheduled_transfer(id: i32, scheduled_transfer_patch: Json<PatchScheduledTransfer>, auth: Authentication) -> Result<Json<ScheduledTransfer>, Status> {
    match DatabaseScheduledTransfers::new().get_scheduled_transfer(id, auth.token.claims.user_id) {
        Ok(scheduled_transfer_tuple) => {
            match DatabaseAccounts::new().get_account(scheduled_transfer_patch.origin_account_id, auth.token.claims.user_id) {
                Ok(_) => {
                    match DatabaseAccounts::new().get_account(scheduled_transfer_patch.destination_account_id, auth.token.claims.user_id) {
                        Ok(_) => {
                            match internal_get_new_scheduled_transfer_from_post_patch(&scheduled_transfer_patch, &auth) {
                                Some(mut new_scheduled_transfer) => {
                                    new_scheduled_transfer.current_repeat_count = scheduled_transfer_tuple.0.current_repeat_count;

                                    match DatabaseScheduledTransfers::new().update_scheduled_transfer(id, &new_scheduled_transfer, auth.token.claims.user_id) {
                                        Ok(scheduled_transfer) => Ok(Json(scheduled_transfer)),
                                        Err(_) => Err(Status::NotFound)
                                    }
                                }
                                None => Err(Status::BadRequest)
                            }
                        }
                        Err(_) => Err(Status::NotFound)
                    }
                }
                Err(_) => Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/scheduled-transfers/<id>")]
pub fn delete_scheduled_transfer(id: i32, auth: Authentication) -> Result<Json<ScheduledTransfer>, Status> {
    match DatabaseScheduledTransfers::new().delete_scheduled_transfer(id, auth.token.claims.user_id) {
        Ok(scheduled_transfer) => Ok(Json(scheduled_transfer)),
        Err(_) => Err(Status::NotFound)
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![
        post_scheduled_transfer,
        post_scheduled_transfer_pay,
        get_scheduled_transfer_with_id,
        patch_scheduled_transfer,
        delete_scheduled_transfer
    ]
}