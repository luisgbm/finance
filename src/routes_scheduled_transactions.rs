use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth_guard::Authentication;
use crate::db_accounts::DatabaseAccounts;
use crate::db_categories::DatabaseCategories;
use crate::db_scheduled_transactions::DatabaseScheduledTransactions;
use crate::db_transactions::DatabaseTransactions;
use crate::models_db::{NewScheduledTransaction, NewTransaction, ScheduledTransaction, Transaction};
use crate::models_routes::{GetScheduledTransaction, PatchScheduledTransaction, PostScheduledTransaction, PostScheduledTransactionPay};
use crate::utils;

#[post("/scheduled-transactions/<scheduled_transaction_id>/pay", format = "json", data = "<transaction>")]
pub fn post_scheduled_transaction_pay(scheduled_transaction_id: i32, transaction: Json<PostScheduledTransactionPay>, auth: Authentication) -> Result<Json<Transaction>, Status> {
    match DatabaseScheduledTransactions::new().get_scheduled_transaction(scheduled_transaction_id, auth.token.claims.user_id) {
        Ok(scheduled_transaction_tuple) => {
            let new_transaction = NewTransaction {
                value: transaction.value,
                description: transaction.description.as_str(),
                date: transaction.date.clone(),
                account: transaction.account,
                category: transaction.category,
                user_id: auth.token.claims.user_id,
            };

            let new_transaction = DatabaseTransactions::new().new_transaction(&new_transaction);

            let scheduled_transaction = scheduled_transaction_tuple.0;

            if scheduled_transaction.repeat == false {
                DatabaseScheduledTransactions::new().delete_scheduled_transaction(scheduled_transaction_id, auth.token.claims.user_id).unwrap();
                return Ok(Json(new_transaction));
            } else {
                let new_repeat_count = scheduled_transaction.current_repeat_count.unwrap() + 1;

                if scheduled_transaction.infinite_repeat.unwrap() == false && new_repeat_count >= scheduled_transaction.end_after_repeats.unwrap() {
                    DatabaseScheduledTransactions::new().delete_scheduled_transaction(scheduled_transaction_id, auth.token.claims.user_id).unwrap();
                    return Ok(Json(new_transaction));
                }

                let new_date = utils::calculate_next_date(
                    scheduled_transaction.created_date,
                    scheduled_transaction.repeat,
                    scheduled_transaction.repeat_freq.unwrap(),
                    scheduled_transaction.repeat_interval.unwrap(),
                    new_repeat_count,
                );

                let scheduled_transaction_paid = NewScheduledTransaction {
                    account_id: scheduled_transaction.account_id,
                    value: scheduled_transaction.value,
                    description: scheduled_transaction.description.as_str(),
                    category_id: scheduled_transaction.category_id,
                    created_date: scheduled_transaction.created_date.clone(),
                    repeat: scheduled_transaction.repeat,
                    repeat_freq: scheduled_transaction.repeat_freq,
                    repeat_interval: scheduled_transaction.repeat_interval,
                    infinite_repeat: scheduled_transaction.infinite_repeat,
                    end_after_repeats: scheduled_transaction.end_after_repeats,
                    current_repeat_count: Some(new_repeat_count),
                    next_date: Some(new_date),
                    user_id: auth.token.claims.user_id,
                };

                match DatabaseScheduledTransactions::new().update_scheduled_transaction(scheduled_transaction_id, &scheduled_transaction_paid, auth.token.claims.user_id) {
                    Ok(_) => Ok(Json(new_transaction)),
                    Err(_) => Err(Status::InternalServerError)
                }
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

fn internal_get_new_scheduled_transaction_from_post_patch<'a>(scheduled_transaction: &'a Json<PostScheduledTransaction>, auth: &'a Authentication) -> Option<NewScheduledTransaction<'a>> {
    let repeat = scheduled_transaction.repeat;
    let mut repeat_freq = None;
    let mut repeat_interval = None;
    let mut infinite_repeat = None;
    let mut end_after_repeats = None;
    let mut current_repeat_count = None;
    let next_date = Some(scheduled_transaction.created_date.clone());

    if repeat == true {
        match scheduled_transaction.repeat_freq {
            Some(value) => {
                repeat_freq = Some(value);
            }
            None => {
                return None;
            }
        }

        match scheduled_transaction.repeat_interval {
            Some(value) => {
                repeat_interval = Some(value);
            }
            None => {
                return None;
            }
        }

        match scheduled_transaction.infinite_repeat {
            Some(value) => {
                infinite_repeat = Some(value);

                if value == true {
                    end_after_repeats = None;
                } else {
                    match scheduled_transaction.end_after_repeats {
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

    Some(NewScheduledTransaction {
        account_id: scheduled_transaction.account_id,
        value: scheduled_transaction.value,
        description: scheduled_transaction.description.as_str(),
        category_id: scheduled_transaction.category_id,
        created_date: scheduled_transaction.created_date.clone(),
        repeat: scheduled_transaction.repeat,
        repeat_freq,
        repeat_interval,
        infinite_repeat,
        end_after_repeats,
        current_repeat_count,
        next_date,
        user_id: auth.token.claims.user_id,
    })
}

#[post("/scheduled-transactions", format = "json", data = "<scheduled_transaction>")]
pub fn post_scheduled_transaction(scheduled_transaction: Json<PostScheduledTransaction>, auth: Authentication) -> Result<Json<ScheduledTransaction>, Status> {
    match DatabaseAccounts::new().get_account(scheduled_transaction.account_id, auth.token.claims.user_id) {
        Ok(_) => {
            match DatabaseCategories::new().get_category(scheduled_transaction.category_id, auth.token.claims.user_id) {
                Ok(_) => {
                    match internal_get_new_scheduled_transaction_from_post_patch(&scheduled_transaction, &auth) {
                        Some(new_scheduled_transaction) => {
                            return Ok(Json(DatabaseScheduledTransactions::new().new_scheduled_transaction(&new_scheduled_transaction)));
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

#[get("/scheduled-transactions")]
pub fn get_scheduled_transactions(auth: Authentication) -> Json<Vec<GetScheduledTransaction>> {
    let scheduled_transactions_tuples = DatabaseScheduledTransactions::new().get_all_scheduled_transactions(auth.token.claims.user_id);

    let mut scheduled_transactions = Vec::new();

    for scheduled_transaction_tuple in &scheduled_transactions_tuples {
        scheduled_transactions.push(utils::create_scheduled_transaction_join(scheduled_transaction_tuple));
    }

    Json(scheduled_transactions)
}

#[get("/scheduled-transactions/<id>")]
pub fn get_scheduled_transaction_with_id(id: i32, auth: Authentication) -> Result<Json<GetScheduledTransaction>, Status> {
    match DatabaseScheduledTransactions::new().get_scheduled_transaction(id, auth.token.claims.user_id) {
        Ok(tuple) => Ok(Json(utils::create_scheduled_transaction_join(&tuple))),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/scheduled-transactions/<id>", format = "json", data = "<scheduled_transaction_patch>")]
pub fn patch_scheduled_transaction(id: i32, scheduled_transaction_patch: Json<PatchScheduledTransaction>, auth: Authentication) -> Result<Json<ScheduledTransaction>, Status> {
    match DatabaseScheduledTransactions::new().get_scheduled_transaction(id, auth.token.claims.user_id) {
        Ok(scheduled_transaction_tuple) => {
            match DatabaseAccounts::new().get_account(scheduled_transaction_patch.account_id, auth.token.claims.user_id) {
                Ok(_) => {
                    match DatabaseCategories::new().get_category(scheduled_transaction_patch.category_id, auth.token.claims.user_id) {
                        Ok(_) => {
                            match internal_get_new_scheduled_transaction_from_post_patch(&scheduled_transaction_patch, &auth) {
                                Some(mut new_scheduled_transaction) => {
                                    new_scheduled_transaction.current_repeat_count = scheduled_transaction_tuple.0.current_repeat_count;

                                    match DatabaseScheduledTransactions::new().update_scheduled_transaction(id, &new_scheduled_transaction, auth.token.claims.user_id) {
                                        Ok(scheduled_transaction) => Ok(Json(scheduled_transaction)),
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

#[delete("/scheduled-transactions/<id>")]
pub fn delete_scheduled_transaction(id: i32, auth: Authentication) -> Result<Json<ScheduledTransaction>, Status> {
    match DatabaseScheduledTransactions::new().delete_scheduled_transaction(id, auth.token.claims.user_id) {
        Ok(scheduled_transaction) => Ok(Json(scheduled_transaction)),
        Err(_) => Err(Status::NotFound)
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![
        post_scheduled_transaction,
        post_scheduled_transaction_pay,
        get_scheduled_transactions,
        get_scheduled_transaction_with_id,
        patch_scheduled_transaction,
        delete_scheduled_transaction
    ]
}