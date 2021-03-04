use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::database::models::{CategoryTypes, NewTransaction, Transaction};
use crate::routes::auth_guard::Authentication;
use crate::routes::db_pool::FinancePgDatabase;
use crate::routes::models::{PatchTransaction, PostTransaction, TransactionTransferJoined};
use crate::utils;

#[post("/transactions/account/<account_id>", format = "json", data = "<transaction>")]
pub fn post_transaction(account_id: i32, transaction: Json<PostTransaction>, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Transaction>, Status> {
    match crate::database::accounts::get_account(account_id, auth.token.claims.user_id, &*connection) {
        Ok(_) => {
            match crate::database::categories::get_category(transaction.category, auth.token.claims.user_id, &*connection) {
                Ok(_) => {
                    let t = NewTransaction {
                        value: transaction.value,
                        description: transaction.description.as_str(),
                        date: transaction.date,
                        account: account_id,
                        category: transaction.category,
                        user_id: auth.token.claims.user_id,
                    };

                    return Ok(Json(crate::database::transactions::new_transaction(&t, &*connection)));
                }
                Err(_) => Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/transactions/account/<account_id>")]
pub fn get_transactions(account_id: i32, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Vec<TransactionTransferJoined>>, Status> {
    match crate::database::accounts::get_account(account_id, auth.token.claims.user_id, &*connection) {
        Ok(_) => {
            let mut transactions = Vec::new();

            let joins = crate::database::transactions::get_all_transactions_of_account_joined(account_id, auth.token.claims.user_id, &*connection);

            for join in &joins {
                transactions.push(utils::create_transaction_join(join, auth.token.claims.user_id));
            }

            let transfers_from = crate::database::transfers::get_transfers_from_account(account_id, auth.token.claims.user_id, &*connection);

            for transfer_from in &transfers_from {
                transactions.push(utils::create_transaction_from_transfer(transfer_from, CategoryTypes::TransferExpense, &*connection));
            }

            let transfers_to = crate::database::transfers::get_transfers_to_account(account_id, auth.token.claims.user_id, &*connection);

            for transfer_to in &transfers_to {
                transactions.push(utils::create_transaction_from_transfer(transfer_to, CategoryTypes::TransferIncome, &*connection));
            }

            transactions.sort_by_key(|t| t.date);
            transactions.reverse();

            return Ok(Json(transactions));
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/transactions/<id>")]
pub fn get_transaction_with_id(id: i32, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<TransactionTransferJoined>, Status> {
    match crate::database::transactions::get_transaction(id, auth.token.claims.user_id, &*connection) {
        Ok(join) => Ok(Json(utils::create_transaction_join(&join, auth.token.claims.user_id))),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/transactions/<id>", format = "json", data = "<transaction>")]
pub fn patch_transaction(id: i32, transaction: Json<PatchTransaction>, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Transaction>, Status> {
    match crate::database::accounts::get_account(transaction.account, auth.token.claims.user_id, &*connection) {
        Ok(_) => {
            match crate::database::categories::get_category(transaction.category, auth.token.claims.user_id, &*connection) {
                Ok(_) => {
                    let transaction = NewTransaction {
                        value: transaction.value,
                        description: transaction.description.as_str(),
                        date: transaction.date.clone(),
                        account: transaction.account,
                        category: transaction.category,
                        user_id: auth.token.claims.user_id,
                    };

                    match crate::database::transactions::update_transaction(id, &transaction, auth.token.claims.user_id, &*connection) {
                        Ok(transaction) => Ok(Json(transaction)),
                        Err(_) => Err(Status::NotFound)
                    }
                }
                Err(_) => Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[delete("/transactions/<id>")]
pub fn delete_transaction(id: i32, auth: Authentication, connection: FinancePgDatabase) -> Result<Json<Transaction>, Status> {
    match crate::database::transactions::delete_transaction(id, auth.token.claims.user_id, &*connection) {
        Ok(transaction) => Ok(Json(transaction)),
        Err(_) => Err(Status::NotFound)
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![
        post_transaction,
        get_transactions,
        get_transaction_with_id,
        patch_transaction,
        delete_transaction
    ]
}