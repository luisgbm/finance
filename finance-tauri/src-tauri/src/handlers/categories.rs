use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::auth::AuthUser;
use crate::db;
use crate::error::AppError;
use crate::models::{Category, CategoryTypes, PatchCategory, PostCategory};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/categories", post(post_category).get(get_categories))
        .route("/api/categories/expense", get(get_expense_categories))
        .route("/api/categories/income", get(get_income_categories))
        .route(
            "/api/categories/{id}",
            get(get_category).patch(patch_category).delete(delete_category),
        )
}

async fn post_category(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<PostCategory>,
) -> Result<Json<Category>, AppError> {
    let category =
        db::categories::insert(&state.pool, body.categorytype, &body.name, auth.user_id).await?;
    Ok(Json(category))
}

async fn get_categories(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<Category>>, AppError> {
    Ok(Json(db::categories::get_all(&state.pool, auth.user_id).await?))
}

async fn get_expense_categories(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<Category>>, AppError> {
    Ok(Json(
        db::categories::get_all_by_type(&state.pool, CategoryTypes::Expense, auth.user_id).await?,
    ))
}

async fn get_income_categories(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<Category>>, AppError> {
    Ok(Json(
        db::categories::get_all_by_type(&state.pool, CategoryTypes::Income, auth.user_id).await?,
    ))
}

async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<Category>, AppError> {
    Ok(Json(db::categories::get(&state.pool, id, auth.user_id).await?))
}

async fn patch_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
    Json(body): Json<PatchCategory>,
) -> Result<Json<Category>, AppError> {
    let category =
        db::categories::update(&state.pool, id, body.categorytype, &body.name, auth.user_id).await?;
    Ok(Json(category))
}

async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<Category>, AppError> {
    Ok(Json(db::categories::delete(&state.pool, id, auth.user_id).await?))
}
