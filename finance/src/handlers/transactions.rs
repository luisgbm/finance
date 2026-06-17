use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::auth::AuthUser;
use crate::db;
use crate::error::AppError;
use crate::models::{
    NewTransactionData, PatchTransaction, PostTransaction, Transaction, TransactionTransferJoined,
};
use crate::service;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/transactions/account/{account_id}",
            post(post_transaction).get(get_transactions),
        )
        .route(
            "/api/transactions/{id}",
            get(get_transaction).patch(patch_transaction).delete(delete_transaction),
        )
}

async fn post_transaction(
    State(state): State<AppState>,
    Path(account_id): Path<i32>,
    auth: AuthUser,
    Json(body): Json<PostTransaction>,
) -> Result<Json<Transaction>, AppError> {
    // Both the account and category must exist and belong to the user (404 otherwise).
    db::accounts::get(&state.pool, account_id, auth.user_id).await?;
    db::categories::get(&state.pool, body.category, auth.user_id).await?;

    let data = NewTransactionData {
        value: body.value,
        description: body.description,
        date: body.date,
        account: account_id,
        category: body.category,
        user_id: auth.user_id,
    };

    Ok(Json(db::transactions::insert(&state.pool, &data).await?))
}

async fn get_transactions(
    State(state): State<AppState>,
    Path(account_id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<Vec<TransactionTransferJoined>>, AppError> {
    // 404 if the account does not exist for this user.
    db::accounts::get(&state.pool, account_id, auth.user_id).await?;

    let mut list = Vec::new();

    for row in db::transactions::get_all_of_account_joined(&state.pool, account_id, auth.user_id).await? {
        list.push(service::tx_join_to_dto(row));
    }
    for row in db::transfers::get_from_account_joined(&state.pool, account_id, auth.user_id).await? {
        list.push(service::transfer_from_to_dto(row, auth.user_id));
    }
    for row in db::transfers::get_to_account_joined(&state.pool, account_id, auth.user_id).await? {
        list.push(service::transfer_to_to_dto(row, auth.user_id));
    }

    // Sort by date ascending then reverse -> descending (stable), matching the original.
    list.sort_by_key(|t| t.date);
    list.reverse();

    Ok(Json(list))
}

async fn get_transaction(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<TransactionTransferJoined>, AppError> {
    let row = db::transactions::get_joined(&state.pool, id, auth.user_id).await?;
    Ok(Json(service::tx_join_to_dto(row)))
}

async fn patch_transaction(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
    Json(body): Json<PatchTransaction>,
) -> Result<Json<Transaction>, AppError> {
    db::accounts::get(&state.pool, body.account, auth.user_id).await?;
    db::categories::get(&state.pool, body.category, auth.user_id).await?;

    let data = NewTransactionData {
        value: body.value,
        description: body.description,
        date: body.date,
        account: body.account,
        category: body.category,
        user_id: auth.user_id,
    };

    Ok(Json(db::transactions::update(&state.pool, id, &data, auth.user_id).await?))
}

async fn delete_transaction(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<Transaction>, AppError> {
    Ok(Json(db::transactions::delete(&state.pool, id, auth.user_id).await?))
}
