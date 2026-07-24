//! Tauri IPC commands — the desktop replacement for the former Axum REST handlers.
//!
//! This desktop build is deliberately **single-user with no authentication**: there are no
//! sessions, tokens, or `user_id` scoping. Every command is a thin wrapper over the shared
//! [`AppState`] pool and the `db`/`service` layers, operating on the one local database.
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
    NewScheduledTransaction, NewTransactionData, NewTransferData, PatchCategory,
    PatchScheduledTransaction, PatchTransaction, PatchTransfer, PostCategory,
    PostScheduledTransaction, PostScheduledTransactionPay, PostTransaction, PostTransfer,
    ScheduledTransaction, ScheduledTransactionKinds, Transaction, TransactionTransferJoined,
    Transfer,
};
use crate::service;
use crate::state::AppState;

// ---------------------------------------------------------------------------------------
// Initial data
// ---------------------------------------------------------------------------------------

/// Fetch the full initial payload (accounts, categories, scheduled transactions). Called once
/// on app start to populate the frontend store.
#[tauri::command]
#[specta::specta]
pub async fn get_initial_data(state: State<'_, AppState>) -> Result<InitialData, AppError> {
    service::build_initial_data(&state.pool).await
}

// ---------------------------------------------------------------------------------------
// Accounts
// ---------------------------------------------------------------------------------------

#[tauri::command]
#[specta::specta]
pub async fn create_account(
    state: State<'_, AppState>,
    name: String,
) -> Result<GetAccount, AppError> {
    let account = db::accounts::insert(&state.pool, &name).await?;
    Ok(GetAccount {
        id: account.id,
        name: account.name,
        balance: 0,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn get_accounts(state: State<'_, AppState>) -> Result<Vec<GetAccount>, AppError> {
    service::accounts_with_balance(&state.pool).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_account(
    state: State<'_, AppState>,
    account_id: i32,
) -> Result<GetAccount, AppError> {
    let account = db::accounts::get(&state.pool, account_id).await?;
    let balance = db::accounts::balance(&state.pool, account.id).await?;
    Ok(GetAccount {
        id: account.id,
        name: account.name,
        balance,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn update_account(
    state: State<'_, AppState>,
    account_id: i32,
    name: String,
) -> Result<GetAccount, AppError> {
    let account = db::accounts::update(&state.pool, account_id, &name).await?;
    let balance = db::accounts::balance(&state.pool, account.id).await?;
    Ok(GetAccount {
        id: account.id,
        name: account.name,
        balance,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn delete_account(
    state: State<'_, AppState>,
    account_id: i32,
) -> Result<Account, AppError> {
    db::accounts::delete(&state.pool, account_id).await
}

// ---------------------------------------------------------------------------------------
// Categories
// ---------------------------------------------------------------------------------------

#[tauri::command]
#[specta::specta]
pub async fn create_category(
    state: State<'_, AppState>,
    req: PostCategory,
) -> Result<Category, AppError> {
    db::categories::insert(&state.pool, req.categorytype, &req.name).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_categories(state: State<'_, AppState>) -> Result<Vec<Category>, AppError> {
    db::categories::get_all(&state.pool).await
}

/// Filter categories by type. Replaces the former `/categories/expense` and
/// `/categories/income` routes; the type is matched case-insensitively.
#[tauri::command]
#[specta::specta]
pub async fn get_categories_by_type(
    state: State<'_, AppState>,
    category_type: String,
) -> Result<Vec<Category>, AppError> {
    let category_type = match category_type.to_lowercase().as_str() {
        "expense" => CategoryTypes::Expense,
        "income" => CategoryTypes::Income,
        _ => return Err(AppError::BadRequest),
    };
    db::categories::get_all_by_type(&state.pool, category_type).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_category(
    state: State<'_, AppState>,
    category_id: i32,
) -> Result<Category, AppError> {
    db::categories::get(&state.pool, category_id).await
}

#[tauri::command]
#[specta::specta]
pub async fn update_category(
    state: State<'_, AppState>,
    category_id: i32,
    req: PatchCategory,
) -> Result<Category, AppError> {
    db::categories::update(&state.pool, category_id, req.categorytype, &req.name).await
}

#[tauri::command]
#[specta::specta]
pub async fn delete_category(
    state: State<'_, AppState>,
    category_id: i32,
) -> Result<Category, AppError> {
    db::categories::delete(&state.pool, category_id).await
}

// ---------------------------------------------------------------------------------------
// Transactions
// ---------------------------------------------------------------------------------------

#[tauri::command]
#[specta::specta]
pub async fn create_transaction(
    state: State<'_, AppState>,
    account_id: i32,
    req: PostTransaction,
) -> Result<Transaction, AppError> {
    // Both the account and category must exist (404 otherwise).
    db::accounts::get(&state.pool, account_id).await?;
    db::categories::get(&state.pool, req.category).await?;

    let data = NewTransactionData {
        value: req.value,
        description: req.description,
        date: req.date,
        account: account_id,
        category: req.category,
    };

    db::transactions::insert(&state.pool, &data).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_transactions_for_account(
    state: State<'_, AppState>,
    account_id: i32,
) -> Result<Vec<TransactionTransferJoined>, AppError> {
    // 404 if the account does not exist.
    db::accounts::get(&state.pool, account_id).await?;

    let mut list = Vec::new();

    for row in db::transactions::get_all_of_account_joined(&state.pool, account_id).await? {
        list.push(service::tx_join_to_dto(row));
    }
    for row in db::transfers::get_from_account_joined(&state.pool, account_id).await? {
        list.push(service::transfer_from_to_dto(row));
    }
    for row in db::transfers::get_to_account_joined(&state.pool, account_id).await? {
        list.push(service::transfer_to_to_dto(row));
    }

    // Sort by date ascending then reverse -> descending (stable), matching the original.
    list.sort_by_key(|t| t.date);
    list.reverse();

    Ok(list)
}

#[tauri::command]
#[specta::specta]
pub async fn get_transaction(
    state: State<'_, AppState>,
    transaction_id: i32,
) -> Result<TransactionTransferJoined, AppError> {
    let row = db::transactions::get_joined(&state.pool, transaction_id).await?;
    Ok(service::tx_join_to_dto(row))
}

#[tauri::command]
#[specta::specta]
pub async fn update_transaction(
    state: State<'_, AppState>,
    transaction_id: i32,
    req: PatchTransaction,
) -> Result<Transaction, AppError> {
    db::accounts::get(&state.pool, req.account).await?;
    db::categories::get(&state.pool, req.category).await?;

    let data = NewTransactionData {
        value: req.value,
        description: req.description,
        date: req.date,
        account: req.account,
        category: req.category,
    };

    db::transactions::update(&state.pool, transaction_id, &data).await
}

#[tauri::command]
#[specta::specta]
pub async fn delete_transaction(
    state: State<'_, AppState>,
    transaction_id: i32,
) -> Result<Transaction, AppError> {
    db::transactions::delete(&state.pool, transaction_id).await
}

// ---------------------------------------------------------------------------------------
// Transfers
// ---------------------------------------------------------------------------------------

#[tauri::command]
#[specta::specta]
pub async fn create_transfer(
    state: State<'_, AppState>,
    origin_account: i32,
    destination_account: i32,
    req: PostTransfer,
) -> Result<Transfer, AppError> {
    // Both endpoints of the transfer must exist (404 otherwise).
    db::accounts::get(&state.pool, origin_account).await?;
    db::accounts::get(&state.pool, destination_account).await?;

    let data = NewTransferData {
        origin_account,
        destination_account,
        value: req.value,
        description: req.description,
        date: req.date,
    };

    db::transfers::insert(&state.pool, &data).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_transfer(
    state: State<'_, AppState>,
    transfer_id: i32,
) -> Result<Transfer, AppError> {
    db::transfers::get(&state.pool, transfer_id).await
}

#[tauri::command]
#[specta::specta]
pub async fn update_transfer(
    state: State<'_, AppState>,
    transfer_id: i32,
    req: PatchTransfer,
) -> Result<Transfer, AppError> {
    db::accounts::get(&state.pool, req.origin_account).await?;
    db::accounts::get(&state.pool, req.destination_account).await?;

    let data = NewTransferData {
        origin_account: req.origin_account,
        destination_account: req.destination_account,
        value: req.value,
        description: req.description,
        date: req.date,
    };

    db::transfers::update(&state.pool, transfer_id, &data).await
}

#[tauri::command]
#[specta::specta]
pub async fn delete_transfer(
    state: State<'_, AppState>,
    transfer_id: i32,
) -> Result<Transfer, AppError> {
    db::transfers::delete(&state.pool, transfer_id).await
}

// ---------------------------------------------------------------------------------------
// Scheduled transactions
// ---------------------------------------------------------------------------------------

#[tauri::command]
#[specta::specta]
pub async fn create_scheduled_transaction(
    state: State<'_, AppState>,
    req: PostScheduledTransaction,
) -> Result<GetScheduledTransaction, AppError> {
    let new = build_new_scheduled(&state.pool, &req)
        .await?
        .ok_or(AppError::BadRequest)?;

    let inserted = db::scheduled_transactions::insert(&state.pool, &new).await?;
    service::enrich_scheduled(&state.pool, &inserted).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_scheduled_transactions(
    state: State<'_, AppState>,
) -> Result<Vec<GetScheduledTransaction>, AppError> {
    service::all_scheduled_enriched(&state.pool).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_scheduled_transaction(
    state: State<'_, AppState>,
    scheduled_transaction_id: i32,
) -> Result<GetScheduledTransaction, AppError> {
    let st = db::scheduled_transactions::get(&state.pool, scheduled_transaction_id).await?;
    service::enrich_scheduled(&state.pool, &st).await
}

#[tauri::command]
#[specta::specta]
pub async fn update_scheduled_transaction(
    state: State<'_, AppState>,
    scheduled_transaction_id: i32,
    req: PatchScheduledTransaction,
) -> Result<GetScheduledTransaction, AppError> {
    // Ensure the scheduled transaction exists (404 otherwise).
    db::scheduled_transactions::get(&state.pool, scheduled_transaction_id).await?;

    let new = build_new_scheduled(&state.pool, &req)
        .await?
        .ok_or(AppError::BadRequest)?;

    let updated =
        db::scheduled_transactions::update(&state.pool, scheduled_transaction_id, &new).await?;
    service::enrich_scheduled(&state.pool, &updated).await
}

#[tauri::command]
#[specta::specta]
pub async fn delete_scheduled_transaction(
    state: State<'_, AppState>,
    scheduled_transaction_id: i32,
) -> Result<ScheduledTransaction, AppError> {
    db::scheduled_transactions::delete(&state.pool, scheduled_transaction_id).await
}

#[tauri::command]
#[specta::specta]
pub async fn pay_scheduled_transaction(
    state: State<'_, AppState>,
    scheduled_transaction_id: i32,
    req: PostScheduledTransactionPay,
) -> Result<ScheduledTransaction, AppError> {
    pay_scheduled_impl(&state.pool, scheduled_transaction_id, &req).await
}

// ---------------------------------------------------------------------------------------
// Helpers (shared by the commands above; also exercised directly by the integration tests)
// ---------------------------------------------------------------------------------------

/// The real transaction/transfer a scheduled payment materialises into.
enum Materialize {
    Transaction(NewTransactionData),
    Transfer(NewTransferData),
}

/// What happens to the schedule itself once it has been paid.
enum Advance {
    /// One-off, or the last occurrence of a finite repeat — remove the schedule.
    Remove,
    /// Repeating — bump the count and next date to the following occurrence.
    Next(NewScheduledTransaction),
}

/// Materialise a scheduled transaction into a real transaction/transfer, then either delete
/// the schedule (one-off / finished) or advance it to its next occurrence.
///
/// The materialise + advance/delete writes run in a **single** database transaction, so a
/// crash can never record the payment without also advancing/removing the schedule (which
/// would let the same occurrence be paid twice). Reference validation happens first as reads.
pub(crate) async fn pay_scheduled_impl(
    pool: &SqlitePool,
    id: i32,
    body: &PostScheduledTransactionPay,
) -> Result<ScheduledTransaction, AppError> {
    let st = db::scheduled_transactions::get(pool, id).await?;

    // 1. Validate the payment against the schedule kind and build the write to perform.
    let materialize = match st.kind {
        ScheduledTransactionKinds::Transaction => {
            let (account_id, category_id) = match (body.account_id, body.category_id) {
                (Some(a), Some(c)) => (a, c),
                _ => return Err(AppError::BadRequest),
            };

            db::accounts::get(pool, account_id)
                .await
                .map_err(|_| AppError::NotFound)?;
            db::categories::get(pool, category_id)
                .await
                .map_err(|_| AppError::NotFound)?;

            Materialize::Transaction(NewTransactionData {
                value: body.value,
                description: body.description.clone(),
                date: body.date,
                account: account_id,
                category: category_id,
            })
        }
        ScheduledTransactionKinds::Transfer => {
            let (origin_id, destination_id) =
                match (body.origin_account_id, body.destination_account_id) {
                    (Some(o), Some(d)) => (o, d),
                    _ => return Err(AppError::BadRequest),
                };

            db::accounts::get(pool, origin_id)
                .await
                .map_err(|_| AppError::BadRequest)?;
            db::accounts::get(pool, destination_id)
                .await
                .map_err(|_| AppError::BadRequest)?;

            Materialize::Transfer(NewTransferData {
                origin_account: origin_id,
                destination_account: destination_id,
                value: body.value,
                description: body.description.clone(),
                date: body.date,
            })
        }
    };

    // 2. Decide the schedule's fate (pure computation, no I/O).
    let advance = compute_advance(&st)?;

    // 3. Apply both effects atomically.
    let mut tx = pool.begin().await?;

    match &materialize {
        Materialize::Transaction(data) => {
            db::transactions::insert_on(&mut tx, data).await?;
        }
        Materialize::Transfer(data) => {
            db::transfers::insert_on(&mut tx, data).await?;
        }
    }

    let result = match &advance {
        Advance::Remove => db::scheduled_transactions::delete_on(&mut tx, id).await?,
        Advance::Next(updated) => {
            db::scheduled_transactions::update_on(&mut tx, id, updated).await?
        }
    };

    tx.commit().await?;

    Ok(result)
}

/// Compute what should happen to a schedule after it is paid, without touching the database.
fn compute_advance(st: &ScheduledTransaction) -> Result<Advance, AppError> {
    if !st.repeat {
        return Ok(Advance::Remove);
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
            return Ok(Advance::Remove);
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

    Ok(Advance::Next(NewScheduledTransaction {
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
    }))
}

/// Validate and assemble a `NewScheduledTransaction` from a request body. Returns `Ok(None)`
/// when the payload is invalid (missing repeat parameters, missing/unknown referenced
/// accounts or category, or origin == destination), which the caller maps to a bad request.
pub(crate) async fn build_new_scheduled(
    pool: &SqlitePool,
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
            let account = match db::accounts::get(pool, account_id).await {
                Ok(account) => account,
                Err(_) => return Ok(None),
            };

            let Some(category_id) = body.category_id else {
                return Ok(None);
            };
            let category = match db::categories::get(pool, category_id).await {
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
            let origin = match db::accounts::get(pool, origin_id).await {
                Ok(account) => account,
                Err(_) => return Ok(None),
            };

            let Some(destination_id) = body.destination_account_id else {
                return Ok(None);
            };
            let destination = match db::accounts::get(pool, destination_id).await {
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
