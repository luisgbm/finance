use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::auth::AuthUser;
use crate::db;
use crate::error::AppError;
use crate::models::{
    GetScheduledTransaction, NewScheduledTransaction, NewTransactionData, NewTransferData,
    PatchScheduledTransaction, PostScheduledTransaction, PostScheduledTransactionPay,
    ScheduledTransaction, ScheduledTransactionKinds,
};
use crate::service;
use crate::state::AppState;
use sqlx::SqlitePool;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/scheduled-transactions",
            post(post_scheduled).get(get_scheduled_list),
        )
        .route(
            "/api/scheduled-transactions/{id}",
            get(get_scheduled).patch(patch_scheduled).delete(delete_scheduled),
        )
        .route("/api/scheduled-transactions/{id}/pay", post(pay_scheduled))
}

async fn post_scheduled(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<PostScheduledTransaction>,
) -> Result<Json<GetScheduledTransaction>, AppError> {
    let new = build_new_scheduled(&body, &state.pool, auth.user_id)
        .await?
        .ok_or(AppError::BadRequest)?;

    let inserted = db::scheduled_transactions::insert(&state.pool, &new).await?;
    Ok(Json(service::enrich_scheduled(&state.pool, &inserted).await?))
}

async fn get_scheduled_list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<GetScheduledTransaction>>, AppError> {
    Ok(Json(service::all_scheduled_enriched(&state.pool, auth.user_id).await?))
}

async fn get_scheduled(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<GetScheduledTransaction>, AppError> {
    let st = db::scheduled_transactions::get(&state.pool, id, auth.user_id).await?;
    Ok(Json(service::enrich_scheduled(&state.pool, &st).await?))
}

async fn patch_scheduled(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
    Json(body): Json<PatchScheduledTransaction>,
) -> Result<Json<GetScheduledTransaction>, AppError> {
    // Ensure the scheduled transaction exists (404 otherwise).
    db::scheduled_transactions::get(&state.pool, id, auth.user_id).await?;

    let new = build_new_scheduled(&body, &state.pool, auth.user_id)
        .await?
        .ok_or(AppError::BadRequest)?;

    let updated = db::scheduled_transactions::update(&state.pool, id, &new, auth.user_id).await?;
    Ok(Json(service::enrich_scheduled(&state.pool, &updated).await?))
}

async fn delete_scheduled(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
) -> Result<Json<ScheduledTransaction>, AppError> {
    Ok(Json(db::scheduled_transactions::delete(&state.pool, id, auth.user_id).await?))
}

async fn pay_scheduled(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    auth: AuthUser,
    Json(body): Json<PostScheduledTransactionPay>,
) -> Result<Json<ScheduledTransaction>, AppError> {
    let pool = &state.pool;
    let user_id = auth.user_id;

    let st = db::scheduled_transactions::get(pool, id, user_id).await?;

    // 1. Materialise the scheduled item into a real transaction or transfer.
    match st.kind {
        ScheduledTransactionKinds::Transaction => {
            let (account_id, category_id) = match (body.account_id, body.category_id) {
                (Some(a), Some(c)) => (a, c),
                _ => return Err(AppError::BadRequest),
            };

            db::accounts::get(pool, account_id, user_id)
                .await
                .map_err(|_| AppError::NotFound)?;
            db::categories::get(pool, category_id, user_id)
                .await
                .map_err(|_| AppError::NotFound)?;

            db::transactions::insert(
                pool,
                &NewTransactionData {
                    value: body.value,
                    description: body.description.clone(),
                    date: body.date,
                    account: account_id,
                    category: category_id,
                    user_id,
                },
            )
            .await?;
        }
        ScheduledTransactionKinds::Transfer => {
            let (origin_id, destination_id) =
                match (body.origin_account_id, body.destination_account_id) {
                    (Some(o), Some(d)) => (o, d),
                    _ => return Err(AppError::BadRequest),
                };

            db::accounts::get(pool, origin_id, user_id)
                .await
                .map_err(|_| AppError::BadRequest)?;
            db::accounts::get(pool, destination_id, user_id)
                .await
                .map_err(|_| AppError::BadRequest)?;

            db::transfers::insert(
                pool,
                &NewTransferData {
                    origin_account: origin_id,
                    destination_account: destination_id,
                    value: body.value,
                    description: body.description.clone(),
                    date: body.date,
                    user_id,
                },
            )
            .await?;
        }
    }

    // 2. Either drop the schedule (one-off / finished) or advance it to the next occurrence.
    if !st.repeat {
        return Ok(Json(db::scheduled_transactions::delete(pool, id, user_id).await?));
    }

    let internal = |m: &str| AppError::Internal(m.to_string());

    let current_repeat_count = st
        .current_repeat_count
        .ok_or_else(|| internal("repeating schedule missing current_repeat_count"))?;
    let infinite_repeat = st
        .infinite_repeat
        .ok_or_else(|| internal("repeating schedule missing infinite_repeat"))?;
    let new_repeat_count = current_repeat_count + 1;

    if !infinite_repeat {
        let end_after_repeats = st
            .end_after_repeats
            .ok_or_else(|| internal("finite schedule missing end_after_repeats"))?;
        if new_repeat_count >= end_after_repeats {
            return Ok(Json(db::scheduled_transactions::delete(pool, id, user_id).await?));
        }
    }

    let repeat_freq = st
        .repeat_freq
        .ok_or_else(|| internal("repeating schedule missing repeat_freq"))?;
    let repeat_interval = st
        .repeat_interval
        .ok_or_else(|| internal("repeating schedule missing repeat_interval"))?;

    let next_date = service::calculate_next_date(
        st.created_date,
        st.repeat,
        repeat_freq,
        repeat_interval,
        new_repeat_count,
    );

    let updated_input = NewScheduledTransaction {
        kind: st.kind,
        value: st.value,
        description: st.description.clone(),
        created_date: st.created_date,
        account_id: st.account_id,
        category_id: st.category_id,
        origin_account_id: st.origin_account_id,
        destination_account_id: st.destination_account_id,
        repeat: st.repeat,
        repeat_freq: st.repeat_freq,
        repeat_interval: st.repeat_interval,
        infinite_repeat: st.infinite_repeat,
        end_after_repeats: st.end_after_repeats,
        current_repeat_count: Some(new_repeat_count),
        next_date: Some(next_date),
        user_id,
    };

    db::scheduled_transactions::update(pool, id, &updated_input, user_id)
        .await
        .map(Json)
        .map_err(|_| internal("failed to update scheduled transaction"))
}

/// Validate and assemble a `NewScheduledTransaction` from a request body. Returns `Ok(None)`
/// when the payload is invalid (missing repeat parameters, missing/unknown referenced
/// accounts or category, or origin == destination), which the caller maps to HTTP 400.
async fn build_new_scheduled(
    body: &PostScheduledTransaction,
    pool: &SqlitePool,
    user_id: i32,
) -> Result<Option<NewScheduledTransaction>, AppError> {
    let mut new = NewScheduledTransaction {
        kind: body.kind,
        value: body.value,
        description: body.description.clone(),
        created_date: body.created_date,
        account_id: None,
        category_id: None,
        origin_account_id: None,
        destination_account_id: None,
        repeat: body.repeat,
        repeat_freq: None,
        repeat_interval: None,
        infinite_repeat: None,
        end_after_repeats: None,
        current_repeat_count: None,
        next_date: Some(body.created_date),
        user_id,
    };

    if body.repeat {
        let Some(freq) = body.repeat_freq else {
            return Ok(None);
        };
        new.repeat_freq = Some(freq);

        let Some(interval) = body.repeat_interval else {
            return Ok(None);
        };
        new.repeat_interval = Some(interval);

        let Some(infinite) = body.infinite_repeat else {
            return Ok(None);
        };
        new.infinite_repeat = Some(infinite);

        if infinite {
            new.end_after_repeats = None;
        } else {
            let Some(end) = body.end_after_repeats else {
                return Ok(None);
            };
            new.end_after_repeats = Some(end);
        }

        new.current_repeat_count = Some(0);
    }

    match body.kind {
        ScheduledTransactionKinds::Transaction => {
            let Some(account_id) = body.account_id else {
                return Ok(None);
            };
            let account = match db::accounts::get(pool, account_id, user_id).await {
                Ok(account) => account,
                Err(_) => return Ok(None),
            };

            let Some(category_id) = body.category_id else {
                return Ok(None);
            };
            let category = match db::categories::get(pool, category_id, user_id).await {
                Ok(category) => category,
                Err(_) => return Ok(None),
            };

            new.account_id = Some(account.id);
            new.category_id = Some(category.id);
        }
        ScheduledTransactionKinds::Transfer => {
            let Some(origin_id) = body.origin_account_id else {
                return Ok(None);
            };
            let origin = match db::accounts::get(pool, origin_id, user_id).await {
                Ok(account) => account,
                Err(_) => return Ok(None),
            };

            let Some(destination_id) = body.destination_account_id else {
                return Ok(None);
            };
            let destination = match db::accounts::get(pool, destination_id, user_id).await {
                Ok(account) => account,
                Err(_) => return Ok(None),
            };

            if destination_id == origin_id {
                return Ok(None);
            }

            new.origin_account_id = Some(origin.id);
            new.destination_account_id = Some(destination.id);
        }
    }

    Ok(Some(new))
}
