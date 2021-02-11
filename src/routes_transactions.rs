use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth_guard::Authentication;
use crate::db_accounts::DatabaseAccounts;
use crate::db_categories::DatabaseCategories;
use crate::db_transactions::DatabaseTransactions;
use crate::db_transfers::DatabaseTransfers;
use crate::models_db::{CategoryTypes, NewTransaction, Transaction};
use crate::models_routes::{PatchTransaction, PostTransaction, TransactionTransferJoined};
use crate::utils;

#[post("/transactions/account/<account_id>", format = "json", data = "<transaction>")]
pub fn post_transaction(account_id: i32, transaction: Json<PostTransaction>, auth: Authentication) -> Result<Json<Transaction>, Status> {
    match DatabaseAccounts::new().get_account(account_id, auth.token.claims.user_id) {
        Ok(_) => {
            match DatabaseCategories::new().get_category(transaction.category, auth.token.claims.user_id) {
                Ok(_) => {
                    let t = NewTransaction {
                        value: transaction.value,
                        description: transaction.description.as_str(),
                        date: transaction.date,
                        account: account_id,
                        category: transaction.category,
                        user_id: auth.token.claims.user_id,
                    };

                    return Ok(Json(DatabaseTransactions::new().new_transaction(&t)));
                }
                Err(_) => Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/transactions/account/<account_id>")]
pub fn get_transactions(account_id: i32, auth: Authentication) -> Result<Json<Vec<TransactionTransferJoined>>, Status> {
    match DatabaseAccounts::new().get_account(account_id, auth.token.claims.user_id) {
        Ok(_) => {
            let mut transactions = Vec::new();

            let joins = DatabaseTransactions::new().get_all_transactions_of_account_joined(account_id, auth.token.claims.user_id);

            for join in &joins {
                transactions.push(utils::create_transaction_join(join, auth.token.claims.user_id));
            }

            let transfers_from = DatabaseTransfers::new().get_transfers_from_account(account_id, auth.token.claims.user_id);

            for transfer_from in &transfers_from {
                transactions.push(utils::create_transaction_from_transfer(transfer_from, CategoryTypes::TransferExpense));
            }

            let transfers_to = DatabaseTransfers::new().get_transfers_to_account(account_id, auth.token.claims.user_id);

            for transfer_to in &transfers_to {
                transactions.push(utils::create_transaction_from_transfer(transfer_to, CategoryTypes::TransferIncome));
            }

            transactions.sort_by_key(|t| t.date);
            transactions.reverse();

            return Ok(Json(transactions));
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[get("/transactions/<id>")]
pub fn get_transaction_with_id(id: i32, auth: Authentication) -> Result<Json<TransactionTransferJoined>, Status> {
    match DatabaseTransactions::new().get_transaction(id, auth.token.claims.user_id) {
        Ok(join) => Ok(Json(utils::create_transaction_join(&join, auth.token.claims.user_id))),
        Err(_) => Err(Status::NotFound)
    }
}

#[patch("/transactions/<id>", format = "json", data = "<transaction>")]
pub fn patch_transaction(id: i32, transaction: Json<PatchTransaction>, auth: Authentication) -> Result<Json<Transaction>, Status> {
    match DatabaseAccounts::new().get_account(transaction.account, auth.token.claims.user_id) {
        Ok(_) => {
            match DatabaseCategories::new().get_category(transaction.category, auth.token.claims.user_id) {
                Ok(_) => {
                    let transaction = NewTransaction {
                        value: transaction.value,
                        description: transaction.description.as_str(),
                        date: transaction.date.clone(),
                        account: transaction.account,
                        category: transaction.category,
                        user_id: auth.token.claims.user_id,
                    };

                    match DatabaseTransactions::new().update_transaction(id, &transaction, auth.token.claims.user_id) {
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
pub fn delete_transaction(id: i32, auth: Authentication) -> Result<Json<Transaction>, Status> {
    match DatabaseTransactions::new().delete_transaction(id, auth.token.claims.user_id) {
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