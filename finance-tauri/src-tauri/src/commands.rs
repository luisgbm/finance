//! Tauri IPC commands — the desktop replacement for the former Axum REST handlers.
//!
//! Each `#[tauri::command]` is a thin wrapper that receives the shared [`AppState`] plus the
//! caller-supplied `user_id` (sent by the frontend in place of the old JWT) and delegates to
//! the unchanged `db` / `service` layers. Non-trivial logic (scheduled-transaction validation
//! and the pay/advance workflow) lives in free helper functions so it can be unit-tested
//! without constructing a Tauri `State`.
//!
//! Argument naming: Tauri passes command arguments as a JSON object with camelCase keys and
//! maps them to the snake_case Rust parameters (e.g. JS `accountId` -> `account_id`). Struct
//! arguments (`req`) are deserialized by serde using the struct's own field names, so the
//! frontend sends snake_case bodies for those.

use sqlx::SqlitePool;
use tauri::State;

use crate::db;
use crate::error::AppError;
use crate::models::{
    Account, Category, CategoryTypes, GetAccount, GetScheduledTransaction, InitialData,
    NewAppUser, NewScheduledTransaction, NewTransactionData, NewTransferData, PatchCategory,
    PatchScheduledTransaction, PatchTransaction, PatchTransfer, PostCategory,
    PostScheduledTransaction, PostScheduledTransactionPay, PostTransaction, PostTransfer,
    ScheduledTransaction, ScheduledTransactionKinds, Transaction, TransactionTransferJoined,
    Transfer,
};
use crate::service;
use crate::state::AppState;

// ---------------------------------------------------------------------------------------
// Auth / session
// ---------------------------------------------------------------------------------------

#[tauri::command]
pub async fn register(
    state: State<'_, AppState>,
    req: NewAppUser,
) -> Result<InitialData, AppError> {
    let user =
        db::users::insert(&state.pool, &req.name, &req.password, state.config.bf_rounds).await?;

    // Re-authenticate with the same credentials to assemble the initial payload,
    // mirroring the original backend.
    match db::users::authenticate(&state.pool, &user.name, &req.password).await? {
        Some(authed) => service::build_initial_data(&state.pool, authed.id).await,
        None => Err(AppError::Unauthorized),
    }
}

#[tauri::command]
pub async fn login(state: State<'_, AppState>, req: NewAppUser) -> Result<InitialData, AppError> {
    match db::users::authenticate(&state.pool, &req.name, &req.password).await? {
        Some(user) => service::build_initial_data(&state.pool, user.id).await,
        None => Err(AppError::Unauthorized),
    }
}

/// Re-fetch the full initial payload for an already-known user (the former `GET /token`).
/// Used on app start to restore the session persisted in `localStorage`.
#[tauri::command]
pub async fn get_initial_data(
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<InitialData, AppError> {
    service::build_initial_data(&state.pool, user_id).await
}

// ---------------------------------------------------------------------------------------
// Accounts
// ---------------------------------------------------------------------------------------

#[tauri::command]
pub async fn create_account(
    state: State<'_, AppState>,
    user_id: i32,
    name: String,
) -> Result<GetAccount, AppError> {
    let account = db::accounts::insert(&state.pool, &name, user_id).await?;
    Ok(GetAccount {
        id: account.id,
        name: account.name,
        balance: 0,
        user_id: account.user_id,
    })
}

#[tauri::command]
pub async fn get_accounts(
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<Vec<GetAccount>, AppError> {
    service::accounts_with_balance(&state.pool, user_id).await
}

#[tauri::command]
pub async fn get_account(
    state: State<'_, AppState>,
    user_id: i32,
    account_id: i32,
) -> Result<GetAccount, AppError> {
    let account = db::accounts::get(&state.pool, account_id, user_id).await?;
    let balance = db::accounts::balance(&state.pool, account.id, user_id).await?;
    Ok(GetAccount {
        id: account.id,
        name: account.name,
        balance,
        user_id: account.user_id,
    })
}

#[tauri::command]
pub async fn update_account(
    state: State<'_, AppState>,
    user_id: i32,
    account_id: i32,
    name: String,
) -> Result<GetAccount, AppError> {
    let account = db::accounts::update(&state.pool, account_id, &name, user_id).await?;
    let balance = db::accounts::balance(&state.pool, account.id, user_id).await?;
    Ok(GetAccount {
        id: account.id,
        name: account.name,
        balance,
        user_id: account.user_id,
    })
}

#[tauri::command]
pub async fn delete_account(
    state: State<'_, AppState>,
    user_id: i32,
    account_id: i32,
) -> Result<Account, AppError> {
    db::accounts::delete(&state.pool, account_id, user_id).await
}

// ---------------------------------------------------------------------------------------
// Categories
// ---------------------------------------------------------------------------------------

#[tauri::command]
pub async fn create_category(
    state: State<'_, AppState>,
    user_id: i32,
    req: PostCategory,
) -> Result<Category, AppError> {
    db::categories::insert(&state.pool, req.categorytype, &req.name, user_id).await
}

#[tauri::command]
pub async fn get_categories(
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<Vec<Category>, AppError> {
    db::categories::get_all(&state.pool, user_id).await
}

/// Filter categories by type. Replaces the former `/categories/expense` and
/// `/categories/income` routes; the type is matched case-insensitively.
#[tauri::command]
pub async fn get_categories_by_type(
    state: State<'_, AppState>,
    user_id: i32,
    category_type: String,
) -> Result<Vec<Category>, AppError> {
    let category_type = match category_type.to_lowercase().as_str() {
        "expense" => CategoryTypes::Expense,
        "income" => CategoryTypes::Income,
        _ => return Err(AppError::BadRequest),
    };
    db::categories::get_all_by_type(&state.pool, category_type, user_id).await
}

#[tauri::command]
pub async fn get_category(
    state: State<'_, AppState>,
    user_id: i32,
    category_id: i32,
) -> Result<Category, AppError> {
    db::categories::get(&state.pool, category_id, user_id).await
}

#[tauri::command]
pub async fn update_category(
    state: State<'_, AppState>,
    user_id: i32,
    category_id: i32,
    req: PatchCategory,
) -> Result<Category, AppError> {
    db::categories::update(&state.pool, category_id, req.categorytype, &req.name, user_id).await
}

#[tauri::command]
pub async fn delete_category(
    state: State<'_, AppState>,
    user_id: i32,
    category_id: i32,
) -> Result<Category, AppError> {
    db::categories::delete(&state.pool, category_id, user_id).await
}

// ---------------------------------------------------------------------------------------
// Transactions
// ---------------------------------------------------------------------------------------

#[tauri::command]
pub async fn create_transaction(
    state: State<'_, AppState>,
    user_id: i32,
    account_id: i32,
    req: PostTransaction,
) -> Result<Transaction, AppError> {
    // Both the account and category must exist and belong to the user (404 otherwise).
    db::accounts::get(&state.pool, account_id, user_id).await?;
    db::categories::get(&state.pool, req.category, user_id).await?;

    let data = NewTransactionData {
        value: req.value,
        description: req.description,
        date: req.date,
        account: account_id,
        category: req.category,
        user_id,
    };

    db::transactions::insert(&state.pool, &data).await
}

#[tauri::command]
pub async fn get_transactions_for_account(
    state: State<'_, AppState>,
    user_id: i32,
    account_id: i32,
) -> Result<Vec<TransactionTransferJoined>, AppError> {
    // 404 if the account does not exist for this user.
    db::accounts::get(&state.pool, account_id, user_id).await?;

    let mut list = Vec::new();

    for row in db::transactions::get_all_of_account_joined(&state.pool, account_id, user_id).await? {
        list.push(service::tx_join_to_dto(row));
    }
    for row in db::transfers::get_from_account_joined(&state.pool, account_id, user_id).await? {
        list.push(service::transfer_from_to_dto(row, user_id));
    }
    for row in db::transfers::get_to_account_joined(&state.pool, account_id, user_id).await? {
        list.push(service::transfer_to_to_dto(row, user_id));
    }

    // Sort by date ascending then reverse -> descending (stable), matching the original.
    list.sort_by_key(|t| t.date);
    list.reverse();

    Ok(list)
}

#[tauri::command]
pub async fn get_transaction(
    state: State<'_, AppState>,
    user_id: i32,
    transaction_id: i32,
) -> Result<TransactionTransferJoined, AppError> {
    let row = db::transactions::get_joined(&state.pool, transaction_id, user_id).await?;
    Ok(service::tx_join_to_dto(row))
}

#[tauri::command]
pub async fn update_transaction(
    state: State<'_, AppState>,
    user_id: i32,
    transaction_id: i32,
    req: PatchTransaction,
) -> Result<Transaction, AppError> {
    db::accounts::get(&state.pool, req.account, user_id).await?;
    db::categories::get(&state.pool, req.category, user_id).await?;

    let data = NewTransactionData {
        value: req.value,
        description: req.description,
        date: req.date,
        account: req.account,
        category: req.category,
        user_id,
    };

    db::transactions::update(&state.pool, transaction_id, &data, user_id).await
}

#[tauri::command]
pub async fn delete_transaction(
    state: State<'_, AppState>,
    user_id: i32,
    transaction_id: i32,
) -> Result<Transaction, AppError> {
    db::transactions::delete(&state.pool, transaction_id, user_id).await
}

// ---------------------------------------------------------------------------------------
// Transfers
// ---------------------------------------------------------------------------------------

#[tauri::command]
pub async fn create_transfer(
    state: State<'_, AppState>,
    user_id: i32,
    origin_account: i32,
    destination_account: i32,
    req: PostTransfer,
) -> Result<Transfer, AppError> {
    // Both endpoints of the transfer must exist for this user (404 otherwise).
    db::accounts::get(&state.pool, origin_account, user_id).await?;
    db::accounts::get(&state.pool, destination_account, user_id).await?;

    let data = NewTransferData {
        origin_account,
        destination_account,
        value: req.value,
        description: req.description,
        date: req.date,
        user_id,
    };

    db::transfers::insert(&state.pool, &data).await
}

#[tauri::command]
pub async fn get_transfer(
    state: State<'_, AppState>,
    user_id: i32,
    transfer_id: i32,
) -> Result<Transfer, AppError> {
    db::transfers::get(&state.pool, transfer_id, user_id).await
}

#[tauri::command]
pub async fn update_transfer(
    state: State<'_, AppState>,
    user_id: i32,
    transfer_id: i32,
    req: PatchTransfer,
) -> Result<Transfer, AppError> {
    db::accounts::get(&state.pool, req.origin_account, user_id).await?;
    db::accounts::get(&state.pool, req.destination_account, user_id).await?;

    let data = NewTransferData {
        origin_account: req.origin_account,
        destination_account: req.destination_account,
        value: req.value,
        description: req.description,
        date: req.date,
        user_id,
    };

    db::transfers::update(&state.pool, transfer_id, &data, user_id).await
}

#[tauri::command]
pub async fn delete_transfer(
    state: State<'_, AppState>,
    user_id: i32,
    transfer_id: i32,
) -> Result<Transfer, AppError> {
    db::transfers::delete(&state.pool, transfer_id, user_id).await
}

// ---------------------------------------------------------------------------------------
// Scheduled transactions
// ---------------------------------------------------------------------------------------

#[tauri::command]
pub async fn create_scheduled_transaction(
    state: State<'_, AppState>,
    user_id: i32,
    req: PostScheduledTransaction,
) -> Result<GetScheduledTransaction, AppError> {
    let new = build_new_scheduled(&state.pool, user_id, &req)
        .await?
        .ok_or(AppError::BadRequest)?;

    let inserted = db::scheduled_transactions::insert(&state.pool, &new).await?;
    service::enrich_scheduled(&state.pool, &inserted).await
}

#[tauri::command]
pub async fn get_scheduled_transactions(
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<Vec<GetScheduledTransaction>, AppError> {
    service::all_scheduled_enriched(&state.pool, user_id).await
}

#[tauri::command]
pub async fn get_scheduled_transaction(
    state: State<'_, AppState>,
    user_id: i32,
    scheduled_transaction_id: i32,
) -> Result<GetScheduledTransaction, AppError> {
    let st = db::scheduled_transactions::get(&state.pool, scheduled_transaction_id, user_id).await?;
    service::enrich_scheduled(&state.pool, &st).await
}

#[tauri::command]
pub async fn update_scheduled_transaction(
    state: State<'_, AppState>,
    user_id: i32,
    scheduled_transaction_id: i32,
    req: PatchScheduledTransaction,
) -> Result<GetScheduledTransaction, AppError> {
    // Ensure the scheduled transaction exists (404 otherwise).
    db::scheduled_transactions::get(&state.pool, scheduled_transaction_id, user_id).await?;

    let new = build_new_scheduled(&state.pool, user_id, &req)
        .await?
        .ok_or(AppError::BadRequest)?;

    let updated =
        db::scheduled_transactions::update(&state.pool, scheduled_transaction_id, &new, user_id)
            .await?;
    service::enrich_scheduled(&state.pool, &updated).await
}

#[tauri::command]
pub async fn delete_scheduled_transaction(
    state: State<'_, AppState>,
    user_id: i32,
    scheduled_transaction_id: i32,
) -> Result<ScheduledTransaction, AppError> {
    db::scheduled_transactions::delete(&state.pool, scheduled_transaction_id, user_id).await
}

#[tauri::command]
pub async fn pay_scheduled_transaction(
    state: State<'_, AppState>,
    user_id: i32,
    scheduled_transaction_id: i32,
    req: PostScheduledTransactionPay,
) -> Result<ScheduledTransaction, AppError> {
    pay_scheduled_impl(&state.pool, user_id, scheduled_transaction_id, &req).await
}

// ---------------------------------------------------------------------------------------
// Helpers (shared by the commands above; also exercised directly by the integration test)
// ---------------------------------------------------------------------------------------

/// Materialise a scheduled transaction into a real transaction/transfer, then either delete
/// the schedule (one-off / finished) or advance it to its next occurrence.
pub(crate) async fn pay_scheduled_impl(
    pool: &SqlitePool,
    user_id: i32,
    id: i32,
    body: &PostScheduledTransactionPay,
) -> Result<ScheduledTransaction, AppError> {
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
        return db::scheduled_transactions::delete(pool, id, user_id).await;
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
            return db::scheduled_transactions::delete(pool, id, user_id).await;
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
        .map_err(|_| internal("failed to update scheduled transaction"))
}

/// Validate and assemble a `NewScheduledTransaction` from a request body. Returns `Ok(None)`
/// when the payload is invalid (missing repeat parameters, missing/unknown referenced
/// accounts or category, or origin == destination), which the caller maps to a bad request.
pub(crate) async fn build_new_scheduled(
    pool: &SqlitePool,
    user_id: i32,
    body: &PostScheduledTransaction,
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
