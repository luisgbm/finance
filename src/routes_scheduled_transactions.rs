use chrono::Duration;
use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth_guard::Authentication;
use crate::db_accounts::DatabaseAccounts;
use crate::db_categories::DatabaseCategories;
use crate::db_scheduled_transactions::DatabaseScheduledTransactions;
use crate::db_transactions::DatabaseTransactions;
use crate::models_db::{NewScheduledTransaction, NewTransaction, RepeatFrequencies, ScheduledTransaction, Transaction};
use crate::models_routes::{GetScheduledTransaction, PatchScheduledTransaction, PostScheduledTransaction};
use crate::utils;

#[post("/scheduled-transactions/<scheduled_transaction_id>/pay")]
pub fn post_scheduled_transaction_pay(scheduled_transaction_id: i32, auth: Authentication) -> Result<Json<(ScheduledTransaction, Transaction)>, Status> {
    match DatabaseScheduledTransactions::new().get_scheduled_transaction(scheduled_transaction_id, auth.token.claims.user_id) {
        Ok(scheduled_transaction_tuple) => {
            let scheduled_transaction = scheduled_transaction_tuple.0;

            let new_transaction = NewTransaction {
                value: scheduled_transaction.value,
                description: scheduled_transaction.description.as_str(),
                date: scheduled_transaction.created_date.clone(),
                account: scheduled_transaction.account_id,
                category: scheduled_transaction.category_id,
                user_id: auth.token.claims.user_id,
            };

            let new_transaction = DatabaseTransactions::new().new_transaction(&new_transaction);

            let new_repeat_count = scheduled_transaction.current_repeat_count.unwrap() + 1;

            if new_repeat_count >= scheduled_transaction.end_after_repeats.unwrap() {
                let deleted_scheduled_transaction = DatabaseScheduledTransactions::new().delete_scheduled_transaction(scheduled_transaction_id, auth.token.claims.user_id).unwrap();

                Ok(Json((deleted_scheduled_transaction, new_transaction)))
            } else {
                let mut new_date = scheduled_transaction.next_date.clone().unwrap();

                if scheduled_transaction.repeat == true {
                    match scheduled_transaction.repeat_freq.unwrap() {
                        RepeatFrequencies::Days => {
                            new_date = new_date + Duration::days((scheduled_transaction.repeat_interval.unwrap() as i64) * 1);
                        }
                        RepeatFrequencies::Weeks => {
                            new_date = new_date + Duration::days((scheduled_transaction.repeat_interval.unwrap() as i64) * 7);
                        }
                        RepeatFrequencies::Months => {
                            new_date = utils::add_months_to_naive_date_time(scheduled_transaction.repeat_interval.unwrap(), &new_date);
                        }
                        RepeatFrequencies::Years => {
                            new_date = utils::add_years_to_naive_date_time(scheduled_transaction.repeat_interval.unwrap(), &new_date);
                        }
                    }
                }

                let scheduled_transaction_paid = NewScheduledTransaction {
                    account_id: scheduled_transaction.account_id,
                    value: scheduled_transaction.value,
                    description: scheduled_transaction.description.as_str(),
                    category_id: scheduled_transaction.category_id,
                    created_date: scheduled_transaction.created_date.clone(),
                    repeat: scheduled_transaction.repeat,
                    repeat_freq: scheduled_transaction.repeat_freq,
                    repeat_interval: scheduled_transaction.repeat_interval,
                    end_after_repeats: scheduled_transaction.end_after_repeats,
                    current_repeat_count: Some(new_repeat_count),
                    next_date: Some(new_date),
                    user_id: auth.token.claims.user_id,
                };

                let scheduled_transaction_paid = DatabaseScheduledTransactions::new().update_scheduled_transaction(scheduled_transaction_id, &scheduled_transaction_paid, auth.token.claims.user_id).unwrap();

                Ok(Json((scheduled_transaction_paid, new_transaction)))
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[post("/scheduled-transactions", format = "json", data = "<scheduled_transaction>")]
pub fn post_scheduled_transaction(scheduled_transaction: Json<PostScheduledTransaction>, auth: Authentication) -> Result<Json<ScheduledTransaction>, Status> {
    match DatabaseAccounts::new().get_account(scheduled_transaction.account_id, auth.token.claims.user_id) {
        Ok(_) => {
            match DatabaseCategories::new().get_category(scheduled_transaction.category_id, auth.token.claims.user_id) {
                Ok(_) => {
                    let scheduled_transaction = NewScheduledTransaction {
                        account_id: scheduled_transaction.account_id,
                        value: scheduled_transaction.value,
                        description: scheduled_transaction.description.as_str(),
                        category_id: scheduled_transaction.category_id,
                        created_date: scheduled_transaction.created_date.clone(),
                        repeat: scheduled_transaction.repeat,
                        repeat_freq: scheduled_transaction.repeat_freq,
                        repeat_interval: scheduled_transaction.repeat_interval,
                        end_after_repeats: scheduled_transaction.end_after_repeats,
                        current_repeat_count: scheduled_transaction.current_repeat_count,
                        next_date: scheduled_transaction.next_date,
                        user_id: auth.token.claims.user_id,
                    };

                    return Ok(Json(DatabaseScheduledTransactions::new().new_scheduled_transaction(&scheduled_transaction)));
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

#[patch("/scheduled-transactions/<id>", format = "json", data = "<scheduled_transaction>")]
pub fn patch_scheduled_transaction(id: i32, scheduled_transaction: Json<PatchScheduledTransaction>, auth: Authentication) -> Result<Json<ScheduledTransaction>, Status> {
    match DatabaseAccounts::new().get_account(scheduled_transaction.account_id, auth.token.claims.user_id) {
        Ok(_) => {
            match DatabaseCategories::new().get_category(scheduled_transaction.category_id, auth.token.claims.user_id) {
                Ok(_) => {
                    let scheduled_transaction = NewScheduledTransaction {
                        account_id: scheduled_transaction.account_id,
                        value: scheduled_transaction.value,
                        description: scheduled_transaction.description.as_str(),
                        category_id: scheduled_transaction.category_id,
                        created_date: scheduled_transaction.created_date.clone(),
                        repeat: scheduled_transaction.repeat,
                        repeat_freq: scheduled_transaction.repeat_freq,
                        repeat_interval: scheduled_transaction.repeat_interval,
                        end_after_repeats: scheduled_transaction.end_after_repeats,
                        current_repeat_count: scheduled_transaction.current_repeat_count,
                        next_date: scheduled_transaction.next_date,
                        user_id: auth.token.claims.user_id,
                    };

                    match DatabaseScheduledTransactions::new().update_scheduled_transaction(id, &scheduled_transaction, auth.token.claims.user_id) {
                        Ok(scheduled_transaction) => Ok(Json(scheduled_transaction)),
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