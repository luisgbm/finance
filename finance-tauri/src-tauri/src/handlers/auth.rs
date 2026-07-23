use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::auth::AuthUser;
use crate::db;
use crate::error::AppError;
use crate::models::{InitialData, NewAppUser};
use crate::service;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/login", post(login))
        .route("/api/token", get(validate_token))
        .route("/api/users", post(post_user))
}

async fn login(
    State(state): State<AppState>,
    Json(body): Json<NewAppUser>,
) -> Result<Json<InitialData>, AppError> {
    match db::users::authenticate(&state.pool, &body.name, &body.password).await? {
        Some(user) => Ok(Json(service::build_initial_data(&state, user.id).await?)),
        None => Err(AppError::Unauthorized),
    }
}

async fn validate_token(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<InitialData>, AppError> {
    Ok(Json(service::build_initial_data(&state, auth.user_id).await?))
}

async fn post_user(
    State(state): State<AppState>,
    Json(body): Json<NewAppUser>,
) -> Result<Json<InitialData>, AppError> {
    let user = db::users::insert(&state.pool, &body.name, &body.password, state.config.bf_rounds).await?;

    // Re-authenticate with the same credentials to assemble the initial payload,
    // mirroring the original backend.
    match db::users::authenticate(&state.pool, &user.name, &body.password).await? {
        Some(authed) => Ok(Json(service::build_initial_data(&state, authed.id).await?)),
        None => Err(AppError::Unauthorized),
    }
}
