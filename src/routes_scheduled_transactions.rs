use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth_guard::Authentication;
use crate::controller_scheduled_transactions;
use crate::db_accounts::DatabaseAccounts;
use crate::db_categories::DatabaseCategories;
use crate::db_scheduled_transactions::DatabaseScheduledTransactions;
use crate::db_transactions::DatabaseTransactions;
use crate::db_transfers::DatabaseTransfers;
use crate::models_db::{NewScheduledTransaction, NewTransaction, NewTransfer, ScheduledTransaction, ScheduledTransactionKinds};
use crate::models_routes::{GetScheduledTransaction, PatchScheduledTransaction, PostScheduledTransaction, PostScheduledTransactionPay};
use crate::utils;

#[post("/scheduled-transactions/<scheduled_transaction_id>/pay", format = "json", data = "<scheduled_transasction_pay>")]
pub fn post_scheduled_transaction_pay(scheduled_transaction_id: i32, scheduled_transasction_pay: Json<PostScheduledTransactionPay>, auth: Authentication) -> Result<Json<ScheduledTransaction>, Status> {
    let scheduled_transaction = DatabaseScheduledTransactions::new().get_scheduled_transaction(scheduled_transaction_id, auth.token.claims.user_id);

    if let Err(_) = scheduled_transaction {
        return Err(Status::NotFound);
    }

    let scheduled_transaction = scheduled_transaction.unwrap();

    match scheduled_transaction.kind {
        ScheduledTransactionKinds::Transaction => {
            if scheduled_transasction_pay.account_id.is_none() || scheduled_transasction_pay.category_id.is_none() {
                return Err(Status::BadRequest);
            }

            if let Err(_) = DatabaseAccounts::new().get_account(scheduled_transasction_pay.account_id.unwrap(), auth.token.claims.user_id) {
                return Err(Status::NotFound);
            }

            if let Err(_) = DatabaseCategories::new().get_category(scheduled_transasction_pay.category_id.unwrap(), auth.token.claims.user_id) {
                return Err(Status::NotFound);
            }

            let new_transaction = NewTransaction {
                value: scheduled_transasction_pay.value,
                description: scheduled_transasction_pay.description.as_str(),
                date: scheduled_transasction_pay.date.clone(),
                account: scheduled_transasction_pay.account_id.unwrap(),
                category: scheduled_transasction_pay.category_id.unwrap(),
                user_id: auth.token.claims.user_id,
            };

            DatabaseTransactions::new().new_transaction(&new_transaction);
        }
        ScheduledTransactionKinds::Transfer => {
            if scheduled_transasction_pay.origin_account_id.is_none() || scheduled_transasction_pay.destination_account_id.is_none() {
                return Err(Status::BadRequest);
            }

            if let Err(_) = DatabaseAccounts::new().get_account(scheduled_transasction_pay.origin_account_id.unwrap(), auth.token.claims.user_id) {
                return Err(Status::BadRequest);
            }

            if let Err(_) = DatabaseAccounts::new().get_account(scheduled_transasction_pay.destination_account_id.unwrap(), auth.token.claims.user_id) {
                return Err(Status::BadRequest);
            }

            let new_transfer = NewTransfer {
                origin_account: scheduled_transasction_pay.origin_account_id.unwrap(),
                destination_account: scheduled_transasction_pay.destination_account_id.unwrap(),
                value: scheduled_transasction_pay.value,
                description: scheduled_transasction_pay.description.as_str(),
                date: scheduled_transasction_pay.date.clone(),
                user_id: auth.token.claims.user_id,
            };

            DatabaseTransfers::new().new_transfer(&new_transfer);
        }
    }

    if scheduled_transaction.repeat == false {
        let deleted = DatabaseScheduledTransactions::new().delete_scheduled_transaction(scheduled_transaction_id, auth.token.claims.user_id);
        return Ok(Json(deleted.unwrap()));
    } else {
        let new_repeat_count = scheduled_transaction.current_repeat_count.unwrap() + 1;

        if scheduled_transaction.infinite_repeat.unwrap() == false && new_repeat_count >= scheduled_transaction.end_after_repeats.unwrap() {
            let deleted = DatabaseScheduledTransactions::new().delete_scheduled_transaction(scheduled_transaction_id, auth.token.claims.user_id);
            return Ok(Json(deleted.unwrap()));
        }

        let new_date = utils::calculate_next_date(
            scheduled_transaction.created_date,
            scheduled_transaction.repeat,
            scheduled_transaction.repeat_freq.unwrap(),
            scheduled_transaction.repeat_interval.unwrap(),
            new_repeat_count,
        );

        let scheduled_transaction_paid = NewScheduledTransaction {
            kind: scheduled_transaction.kind,
            value: scheduled_transaction.value,
            description: scheduled_transaction.description.clone(),
            account_id: scheduled_transaction.account_id,
            category_id: scheduled_transaction.category_id,
            origin_account_id: scheduled_transaction.origin_account_id,
            destination_account_id: scheduled_transaction.destination_account_id,
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
            Ok(updated) => Ok(Json(updated)),
            Err(_) => Err(Status::InternalServerError)
        }
    }
}

fn internal_get_new_scheduled_transaction_for_post_patch(scheduled_transaction: &Json<PostScheduledTransaction>, auth: &Authentication) -> Option<NewScheduledTransaction> {
    let mut new_scheduled_transaction = NewScheduledTransaction {
        kind: scheduled_transaction.kind,
        value: scheduled_transaction.value,
        description: scheduled_transaction.description.clone(),
        created_date: scheduled_transaction.created_date.clone(),
        account_id: None,
        category_id: None,
        origin_account_id: None,
        destination_account_id: None,
        repeat: scheduled_transaction.repeat,
        repeat_freq: None,
        repeat_interval: None,
        infinite_repeat: None,
        end_after_repeats: None,
        current_repeat_count: None,
        next_date: Some(scheduled_transaction.created_date.clone()),
        user_id: auth.token.claims.user_id,
    };

    if scheduled_transaction.repeat == true {
        if scheduled_transaction.repeat_freq.is_none() {
            return None;
        }

        new_scheduled_transaction.repeat_freq = scheduled_transaction.repeat_freq;

        if scheduled_transaction.repeat_interval.is_none() {
            return None;
        }

        new_scheduled_transaction.repeat_interval = scheduled_transaction.repeat_interval;

        if scheduled_transaction.infinite_repeat.is_none() {
            return None;
        }

        new_scheduled_transaction.infinite_repeat = scheduled_transaction.infinite_repeat;

        if scheduled_transaction.infinite_repeat.unwrap() == true {
            new_scheduled_transaction.end_after_repeats = None;
        } else {
            if scheduled_transaction.end_after_repeats.is_none() {
                return None;
            }

            new_scheduled_transaction.end_after_repeats = scheduled_transaction.end_after_repeats;
        }

        new_scheduled_transaction.current_repeat_count = Some(0);
    }

    match scheduled_transaction.kind {
        ScheduledTransactionKinds::Transaction => {
            if scheduled_transaction.account_id.is_none() {
                return None;
            }

            let account_id = scheduled_transaction.account_id.unwrap();

            let account = DatabaseAccounts::new().get_account(account_id, auth.token.claims.user_id);

            if let Err(_) = account {
                return None;
            }

            let account = account.unwrap();

            if scheduled_transaction.category_id.is_none() {
                return None;
            }

            let category_id = scheduled_transaction.category_id.unwrap();

            let category = DatabaseCategories::new().get_category(category_id, auth.token.claims.user_id);

            if let Err(_) = category {
                return None;
            }

            let category = category.unwrap();

            new_scheduled_transaction.account_id = Some(account.id);
            new_scheduled_transaction.category_id = Some(category.id);
        }
        ScheduledTransactionKinds::Transfer => {
            if scheduled_transaction.origin_account_id.is_none() {
                return None;
            }

            let origin_account_id = scheduled_transaction.origin_account_id.unwrap();

            let origin_account = DatabaseAccounts::new().get_account(origin_account_id, auth.token.claims.user_id);

            if let Err(_) = origin_account {
                return None;
            }

            let origin_account = origin_account.unwrap();

            if scheduled_transaction.destination_account_id.is_none() {
                return None;
            }

            let destination_account_id = scheduled_transaction.destination_account_id.unwrap();

            let destination_account = DatabaseAccounts::new().get_account(destination_account_id, auth.token.claims.user_id);

            if let Err(_) = destination_account {
                return None;
            }

            let destination_account = destination_account.unwrap();

            if destination_account_id == origin_account_id {
                return None;
            }

            new_scheduled_transaction.origin_account_id = Some(origin_account.id);
            new_scheduled_transaction.destination_account_id = Some(destination_account.id);
        }
    }

    Some(new_scheduled_transaction)
}

#[post("/scheduled-transactions", format = "json", data = "<scheduled_transaction>")]
pub fn post_scheduled_transaction(scheduled_transaction: Json<PostScheduledTransaction>, auth: Authentication) -> Result<Json<GetScheduledTransaction>, Status> {
    let new_scheduled_transaction = internal_get_new_scheduled_transaction_for_post_patch(&scheduled_transaction, &auth);

    if new_scheduled_transaction.is_none() {
        return Err(Status::BadRequest);
    }

    let new_scheduled_transaction = DatabaseScheduledTransactions::new().new_scheduled_transaction(&new_scheduled_transaction.unwrap());

    let get_scheduled_transaction = utils::create_scheduled_transaction_join(&new_scheduled_transaction);

    if get_scheduled_transaction.is_none() {
        return Err(Status::InternalServerError);
    }

    Ok(Json(get_scheduled_transaction.unwrap()))
}

#[patch("/scheduled-transactions/<id>", format = "json", data = "<scheduled_transaction_patch>")]
pub fn patch_scheduled_transaction(id: i32, scheduled_transaction_patch: Json<PatchScheduledTransaction>, auth: Authentication) -> Result<Json<GetScheduledTransaction>, Status> {
    let scheduled_transaction = DatabaseScheduledTransactions::new().get_scheduled_transaction(id, auth.token.claims.user_id);

    if let Err(_) = scheduled_transaction {
        return Err(Status::NotFound);
    }

    let _scheduled_transaction = scheduled_transaction.unwrap();

    let new_scheduled_transaction = internal_get_new_scheduled_transaction_for_post_patch(&scheduled_transaction_patch, &auth);

    if new_scheduled_transaction.is_none() {
        return Err(Status::BadRequest);
    }

    let new_scheduled_transaction = new_scheduled_transaction.unwrap();

    let updated_scheduled_transaction = DatabaseScheduledTransactions::new().update_scheduled_transaction(id, &new_scheduled_transaction, auth.token.claims.user_id);

    if let Err(_) = updated_scheduled_transaction {
        return Err(Status::InternalServerError);
    }

    let updated_scheduled_transaction = updated_scheduled_transaction.unwrap();

    let get_scheduled_transaction = utils::create_scheduled_transaction_join(&updated_scheduled_transaction);

    if get_scheduled_transaction.is_none() {
        return Err(Status::InternalServerError);
    }

    Ok(Json(get_scheduled_transaction.unwrap()))
}

#[get("/scheduled-transactions")]
pub fn get_scheduled_transactions(auth: Authentication) -> Result<Json<Vec<GetScheduledTransaction>>, Status> {
    if let Some(scheduled_transactions) = controller_scheduled_transactions::get_all_scheduled_transactions(auth.token.claims.user_id) {
        return Ok(Json(scheduled_transactions));
    }

    Err(Status::InternalServerError)
}

#[get("/scheduled-transactions/<id>")]
pub fn get_scheduled_transaction_with_id(id: i32, auth: Authentication) -> Result<Json<GetScheduledTransaction>, Status> {
    let scheduled_transaction = DatabaseScheduledTransactions::new().get_scheduled_transaction(id, auth.token.claims.user_id);

    if let Err(_) = scheduled_transaction {
        return Err(Status::NotFound);
    }

    let scheduled_transaction = scheduled_transaction.unwrap();

    let get_scheduled_transaction = utils::create_scheduled_transaction_join(&scheduled_transaction);

    if get_scheduled_transaction.is_none() {
        return Err(Status::InternalServerError);
    }

    Ok(Json(get_scheduled_transaction.unwrap()))
}

#[delete("/scheduled-transactions/<id>")]
pub fn delete_scheduled_transaction(id: i32, auth: Authentication) -> Result<Json<ScheduledTransaction>, Status> {
    let scheduled_transaction = DatabaseScheduledTransactions::new().delete_scheduled_transaction(id, auth.token.claims.user_id);

    if let Err(_) = scheduled_transaction {
        return Err(Status::NotFound);
    }

    Ok(Json(scheduled_transaction.unwrap()))
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