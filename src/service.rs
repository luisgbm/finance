use chrono::{Duration, NaiveDateTime};
use chronoutil::RelativeDuration;
use sqlx::PgPool;

use crate::auth;
use crate::db;
use crate::db::transactions::TxJoinRow;
use crate::db::transfers::{TransferFromRow, TransferToRow};
use crate::error::AppError;
use crate::models::{
    CategoryTypes, GetAccount, GetScheduledTransaction, InitialData, RepeatFrequencies,
    ScheduledTransaction, ScheduledTransactionKinds, TransactionTransferJoined,
};
use crate::state::AppState;

/// Load all of a user's accounts, each with its computed balance.
pub async fn accounts_with_balance(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<GetAccount>, AppError> {
    let accounts = db::accounts::get_all(pool, user_id).await?;
    let mut result = Vec::with_capacity(accounts.len());

    for account in &accounts {
        let balance = db::accounts::balance(pool, account.id, user_id).await?;
        result.push(GetAccount {
            id: account.id,
            name: account.name.clone(),
            balance,
            user_id,
        });
    }

    Ok(result)
}

/// Build the `InitialData` payload returned by login / register / token-refresh.
pub async fn build_initial_data(
    state: &AppState,
    user_id: i32,
) -> Result<InitialData, AppError> {
    let token = auth::create_jwt(
        user_id,
        &state.config.jwt_secret,
        state.config.jwt_validity_days,
    )?;

    Ok(InitialData {
        token,
        accounts: accounts_with_balance(&state.pool, user_id).await?,
        categories: db::categories::get_all(&state.pool, user_id).await?,
        scheduled_transactions: all_scheduled_enriched(&state.pool, user_id).await?,
    })
}

/// Load all of a user's scheduled transactions, enriched with account/category names,
/// ordered by `created_date` descending (the order comes from the database query).
pub async fn all_scheduled_enriched(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<GetScheduledTransaction>, AppError> {
    let scheduled = db::scheduled_transactions::get_all(pool, user_id).await?;
    let mut result = Vec::with_capacity(scheduled.len());

    for st in &scheduled {
        result.push(enrich_scheduled(pool, st).await?);
    }

    Ok(result)
}

/// Enrich a scheduled transaction with the names/types of its referenced accounts and
/// category. A missing reference is treated as an internal error (HTTP 500), matching the
/// original behaviour where an unresolved join aborted the whole request.
pub async fn enrich_scheduled(
    pool: &PgPool,
    st: &ScheduledTransaction,
) -> Result<GetScheduledTransaction, AppError> {
    let mut dto = GetScheduledTransaction {
        id: st.id,
        kind: st.kind,
        value: st.value,
        description: st.description.clone(),
        created_date: st.created_date,
        account_id: None,
        account_name: None,
        category_id: None,
        category_type: None,
        category_name: None,
        origin_account_id: None,
        origin_account_name: None,
        destination_account_id: None,
        destination_account_name: None,
        repeat: st.repeat,
        repeat_freq: st.repeat_freq,
        repeat_interval: st.repeat_interval,
        infinite_repeat: st.infinite_repeat,
        end_after_repeats: st.end_after_repeats,
        current_repeat_count: st.current_repeat_count,
        next_date: st.next_date,
        user_id: st.user_id,
    };

    let missing = || AppError::Internal("scheduled transaction has an unresolved reference".into());

    match st.kind {
        ScheduledTransactionKinds::Transaction => {
            let account_id = st.account_id.ok_or_else(missing)?;
            let category_id = st.category_id.ok_or_else(missing)?;

            let account = db::accounts::get(pool, account_id, st.user_id)
                .await
                .map_err(|_| missing())?;
            let category = db::categories::get(pool, category_id, st.user_id)
                .await
                .map_err(|_| missing())?;

            dto.account_id = Some(account.id);
            dto.account_name = Some(account.name);
            dto.category_id = Some(category.id);
            dto.category_type = Some(category.categorytype);
            dto.category_name = Some(category.name);
        }
        ScheduledTransactionKinds::Transfer => {
            let origin_id = st.origin_account_id.ok_or_else(missing)?;
            let destination_id = st.destination_account_id.ok_or_else(missing)?;

            let origin = db::accounts::get(pool, origin_id, st.user_id)
                .await
                .map_err(|_| missing())?;
            let destination = db::accounts::get(pool, destination_id, st.user_id)
                .await
                .map_err(|_| missing())?;

            dto.origin_account_id = Some(origin.id);
            dto.origin_account_name = Some(origin.name);
            dto.destination_account_id = Some(destination.id);
            dto.destination_account_name = Some(destination.name);
        }
    }

    Ok(dto)
}

/// Map a joined transaction row to the unified transaction/transfer response shape.
pub fn tx_join_to_dto(row: TxJoinRow) -> TransactionTransferJoined {
    TransactionTransferJoined {
        id: row.id,
        value: row.value,
        description: row.description,
        date: row.date,
        category_id: Some(row.category_id),
        category_type: row.category_type,
        category_name: Some(row.category_name),
        account_id: row.account_id,
        account_name: row.account_name,
        user_id: row.user_id,
        from_account_id: None,
        from_account_name: None,
    }
}

/// Map a transfer leaving the viewed account to a `TransferExpense` pseudo-transaction.
pub fn transfer_from_to_dto(row: TransferFromRow, user_id: i32) -> TransactionTransferJoined {
    TransactionTransferJoined {
        id: row.id,
        value: row.value,
        description: row.description,
        date: row.date,
        category_id: None,
        category_type: CategoryTypes::TransferExpense,
        category_name: None,
        account_id: row.origin_account,
        account_name: row.origin_name.clone(),
        user_id,
        from_account_id: Some(row.origin_account),
        from_account_name: Some(row.origin_name),
    }
}

/// Map a transfer entering the viewed account to a `TransferIncome` pseudo-transaction.
pub fn transfer_to_to_dto(row: TransferToRow, user_id: i32) -> TransactionTransferJoined {
    TransactionTransferJoined {
        id: row.id,
        value: row.value,
        description: row.description,
        date: row.date,
        category_id: None,
        category_type: CategoryTypes::TransferIncome,
        category_name: None,
        account_id: row.destination_account,
        account_name: row.dest_name,
        user_id,
        from_account_id: Some(row.origin_account),
        from_account_name: Some(row.origin_name),
    }
}

/// Compute the next due date for a repeating scheduled transaction.
pub fn calculate_next_date(
    initial_date: NaiveDateTime,
    repeat: bool,
    repeat_freq: RepeatFrequencies,
    repeat_interval: i32,
    current_repeat_count: i32,
) -> NaiveDateTime {
    if !repeat {
        return initial_date;
    }

    match repeat_freq {
        RepeatFrequencies::Days => {
            initial_date + RelativeDuration::days((current_repeat_count * repeat_interval) as i64)
        }
        RepeatFrequencies::Weeks => {
            initial_date + Duration::weeks((current_repeat_count * repeat_interval) as i64)
        }
        RepeatFrequencies::Months => {
            initial_date + RelativeDuration::months(current_repeat_count * repeat_interval)
        }
        RepeatFrequencies::Years => {
            initial_date + RelativeDuration::years(current_repeat_count * repeat_interval)
        }
    }
}
