use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::auth::AuthUser;
use crate::db;
use crate::error::AppError;
use crate::models::{Account, GetAccount, PatchAccount, PostAccount};
use crate::service;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/accounts", post(post_account).get(get_accounts))
        .route(
            "/api/accounts/{id}",
            get(get_account).patch(patch_account).delete(delete_account),
        )
}

async fn post_account(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<PostAccount>,
) -> Result<Json<GetAccount>, AppError> {
    let account = db::accounts::insert(&state.pool, &body.name, auth.user_id).await?;
    Ok(Json(GetAccount {
        id: account.id,
        name: account.name,
        balance: 0,
        user_id: account.user_id,
    }))
}

async fn get_accounts(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<GetAccount>>, AppError> {
    Ok(Json(service::accounts_with_balance(&state.pool, auth.user_id).await?))
}

async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<GetAccount>, AppError> {
    let account = db::accounts::get(&state.pool, id, auth.user_id).await?;
    let balance = db::accounts::balance(&state.pool, account.id, auth.user_id).await?;
    Ok(Json(GetAccount {
        id: account.id,
        name: account.name,
        balance,
        user_id: account.user_id,
    }))
}

async fn patch_account(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
    Json(body): Json<PatchAccount>,
) -> Result<Json<GetAccount>, AppError> {
    let account = db::accounts::update(&state.pool, id, &body.name, auth.user_id).await?;
    let balance = db::accounts::balance(&state.pool, account.id, auth.user_id).await?;
    Ok(Json(GetAccount {
        id: account.id,
        name: account.name,
        balance,
        user_id: account.user_id,
    }))
}

async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<Account>, AppError> {
    Ok(Json(db::accounts::delete(&state.pool, id, auth.user_id).await?))
}
