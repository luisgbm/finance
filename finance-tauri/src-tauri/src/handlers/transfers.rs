use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::auth::AuthUser;
use crate::db;
use crate::error::AppError;
use crate::models::{NewTransferData, PatchTransfer, PostTransfer, Transfer};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/transfers/from/{origin_account}/to/{destination_account}",
            post(post_transfer),
        )
        .route(
            "/api/transfers/{id}",
            get(get_transfer).patch(patch_transfer).delete(delete_transfer),
        )
}

async fn post_transfer(
    State(state): State<AppState>,
    Path((origin_account, destination_account)): Path<(i32, i32)>,
    auth: AuthUser,
    Json(body): Json<PostTransfer>,
) -> Result<Json<Transfer>, AppError> {
    // Both endpoints of the transfer must exist for this user (404 otherwise).
    db::accounts::get(&state.pool, origin_account, auth.user_id).await?;
    db::accounts::get(&state.pool, destination_account, auth.user_id).await?;

    let data = NewTransferData {
        origin_account,
        destination_account,
        value: body.value,
        description: body.description,
        date: body.date,
        user_id: auth.user_id,
    };

    Ok(Json(db::transfers::insert(&state.pool, &data).await?))
}

async fn get_transfer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<Transfer>, AppError> {
    Ok(Json(db::transfers::get(&state.pool, id, auth.user_id).await?))
}

async fn patch_transfer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
    Json(body): Json<PatchTransfer>,
) -> Result<Json<Transfer>, AppError> {
    db::accounts::get(&state.pool, body.origin_account, auth.user_id).await?;
    db::accounts::get(&state.pool, body.destination_account, auth.user_id).await?;

    let data = NewTransferData {
        origin_account: body.origin_account,
        destination_account: body.destination_account,
        value: body.value,
        description: body.description,
        date: body.date,
        user_id: auth.user_id,
    };

    Ok(Json(db::transfers::update(&state.pool, id, &data, auth.user_id).await?))
}

async fn delete_transfer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<Transfer>, AppError> {
    Ok(Json(db::transfers::delete(&state.pool, id, auth.user_id).await?))
}
