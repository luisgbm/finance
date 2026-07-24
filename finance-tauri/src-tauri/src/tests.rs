//! End-to-end integration test for the embedded SQLite backend, exercised directly through
//! the `db` / `service` / command-helper layers (there is no HTTP server any more).
//!
//! It drives the full flow against a throwaway SQLite database — accounts -> categories ->
//! transactions -> balance -> transfers -> scheduled transaction -> pay -> foreign-key
//! cascade — validating the whole Postgres->SQLite port (placeholders, enum-as-TEXT, the
//! shared transaction/transfer id sequence, balance computation and FK cascades) plus the
//! scheduled-transaction build/pay logic that the Tauri commands delegate to.
//!
//! This desktop build is single-user with no authentication, so there are no users, sessions
//! or `user_id` scoping to exercise here.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::NaiveDateTime;
use sqlx::SqlitePool;

use crate::bootstrap;
use crate::commands::{build_new_scheduled, pay_scheduled_impl};
use crate::db;
use crate::error::AppError;
use crate::models::{
    CategoryTypes, NewTransactionData, NewTransferData, PostScheduledTransaction,
    PostScheduledTransactionPay, RepeatFrequencies, ScheduledTransactionKinds,
};
use crate::service;

/// Open a fresh, isolated database in the OS temp dir.
///
/// The filename combines a timestamp with a process-wide counter: tests run in parallel and
/// Windows' `SystemTime` granularity is coarse (~15 ms), so two tests can observe the same
/// nanosecond value — the counter guarantees a distinct file (and thus no lock contention).
async fn fresh_pool() -> SqlitePool {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
    let db_path = std::env::temp_dir().join(format!("finance-tauri-test-{nanos}-{unique}.db"));
    bootstrap::init(&db_path).await.expect("failed to init db")
}

fn dt(value: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S").unwrap()
}

#[tokio::test]
async fn full_flow() {
    let pool = fresh_pool().await;

    // ----- initial payload --------------------------------------------------------------
    let initial = service::build_initial_data(&pool).await.unwrap();
    assert!(initial.accounts.is_empty());
    assert!(initial.categories.is_empty());
    assert!(initial.scheduled_transactions.is_empty());

    // ----- accounts ---------------------------------------------------------------------
    let checking = db::accounts::insert(&pool, "Checking").await.unwrap();
    let savings = db::accounts::insert(&pool, "Savings").await.unwrap();

    // ----- categories (enum stored as TEXT) ---------------------------------------------
    let salary = db::categories::insert(&pool, CategoryTypes::Income, "Salary")
        .await
        .unwrap();
    let groceries = db::categories::insert(&pool, CategoryTypes::Expense, "Groceries")
        .await
        .unwrap();
    let income_cats = db::categories::get_all_by_type(&pool, CategoryTypes::Income)
        .await
        .unwrap();
    assert_eq!(income_cats.len(), 1, "type filter returns only the income category");

    // ----- transactions -----------------------------------------------------------------
    let income_tx = db::transactions::insert(
        &pool,
        &NewTransactionData {
            value: 100_000,
            description: "January salary".into(),
            date: dt("2024-01-05T09:00:00"),
            account: checking.id,
            category: salary.id,
        },
    )
    .await
    .unwrap();
    db::transactions::insert(
        &pool,
        &NewTransactionData {
            value: 3_000,
            description: "Groceries".into(),
            date: dt("2024-01-06T18:30:00"),
            account: checking.id,
            category: groceries.id,
        },
    )
    .await
    .unwrap();

    assert_eq!(
        db::accounts::balance(&pool, checking.id).await.unwrap(),
        97_000,
        "income - expense"
    );

    // ----- transfer (shares the id sequence with transactions) --------------------------
    let transfer = db::transfers::insert(
        &pool,
        &NewTransferData {
            origin_account: checking.id,
            destination_account: savings.id,
            value: 5_000,
            description: "Move to savings".into(),
            date: dt("2024-01-07T12:00:00"),
        },
    )
    .await
    .unwrap();
    assert_ne!(transfer.id, income_tx.id, "transfer/transaction ids must not collide");

    assert_eq!(db::accounts::balance(&pool, checking.id).await.unwrap(), 92_000);
    assert_eq!(db::accounts::balance(&pool, savings.id).await.unwrap(), 5_000);

    // Merged account view: 2 transactions + 1 outgoing transfer, all with distinct ids.
    let mut list = Vec::new();
    for row in db::transactions::get_all_of_account_joined(&pool, checking.id)
        .await
        .unwrap()
    {
        list.push(service::tx_join_to_dto(row));
    }
    for row in db::transfers::get_from_account_joined(&pool, checking.id)
        .await
        .unwrap()
    {
        list.push(service::transfer_from_to_dto(row));
    }
    for row in db::transfers::get_to_account_joined(&pool, checking.id)
        .await
        .unwrap()
    {
        list.push(service::transfer_to_to_dto(row));
    }
    assert_eq!(list.len(), 3, "2 transactions + 1 transfer");
    let mut ids: Vec<i32> = list.iter().map(|t| t.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 3, "all ids in the merged view are unique");

    // ----- scheduled transaction + pay (command helpers) --------------------------------
    let body = PostScheduledTransaction {
        kind: ScheduledTransactionKinds::Transaction,
        value: 2_000,
        description: Some("One-off scheduled expense".into()),
        created_date: dt("2024-01-10T00:00:00"),
        account_id: Some(checking.id),
        category_id: Some(groceries.id),
        origin_account_id: None,
        destination_account_id: None,
        repeat: false,
        repeat_freq: None,
        repeat_interval: None,
        infinite_repeat: None,
        end_after_repeats: None,
        current_repeat_count: None,
        next_date: None,
    };
    let new = build_new_scheduled(&pool, &body)
        .await
        .unwrap()
        .expect("valid schedule");
    let inserted = db::scheduled_transactions::insert(&pool, &new).await.unwrap();
    let enriched = service::enrich_scheduled(&pool, &inserted).await.unwrap();
    assert_eq!(enriched.account_name.as_deref(), Some("Checking"));

    // Paying a one-off schedule creates a real 2000 expense and removes the schedule.
    let pay = PostScheduledTransactionPay {
        value: 2_000,
        description: "Paid scheduled expense".into(),
        date: dt("2024-01-10T10:00:00"),
        category_id: Some(groceries.id),
        account_id: Some(checking.id),
        origin_account_id: None,
        destination_account_id: None,
    };
    pay_scheduled_impl(&pool, inserted.id, &pay).await.unwrap();

    assert!(
        matches!(
            db::scheduled_transactions::get(&pool, inserted.id).await,
            Err(AppError::NotFound)
        ),
        "paid one-off schedule is removed"
    );
    assert_eq!(
        db::accounts::balance(&pool, checking.id).await.unwrap(),
        90_000,
        "balance after pay"
    );

    // ----- foreign-key cascade ----------------------------------------------------------
    db::accounts::delete(&pool, savings.id).await.unwrap();
    assert!(
        matches!(
            db::accounts::get(&pool, savings.id).await,
            Err(AppError::NotFound)
        ),
        "deleted account is gone"
    );
}

/// The IPC error contract: every `AppError` a command can return must serialize to
/// `{ "status": <code>, "message": <text> }` — the shape Tauri sends across the boundary and
/// that the React code inspects via `err.response.status` (e.g. 409 -> already exists).
#[test]
fn app_error_serializes_to_status_and_message() {
    let cases = [
        (AppError::NotFound, 404, "not found"),
        (AppError::Conflict, 409, "conflict"),
        (AppError::BadRequest, 400, "bad request"),
    ];

    for (err, status, message) in cases {
        let value = serde_json::to_value(&err).unwrap();
        assert_eq!(value["status"], status, "status code for '{message}'");
        assert_eq!(value["message"], message, "message for status {status}");
    }

    // The internal variant reports a generic 500 (its detail is logged, not exposed as status).
    let internal = serde_json::to_value(AppError::Internal("boom".into())).unwrap();
    assert_eq!(internal["status"], 500);
    assert_eq!(internal["message"], "internal server error: boom");
}

/// Paying a finite repeating schedule must advance it (count + next date) on each occurrence,
/// then remove it once the final occurrence is paid — and each payment's transaction + the
/// schedule change must land together (the pay path runs them in one database transaction).
#[tokio::test]
async fn paying_a_repeating_schedule_advances_then_finishes() {
    let pool = fresh_pool().await;
    let wallet = db::accounts::insert(&pool, "Wallet").await.unwrap();
    let bills = db::categories::insert(&pool, CategoryTypes::Expense, "Bills")
        .await
        .unwrap();

    // A monthly expense scheduled to occur 3 times.
    let body = PostScheduledTransaction {
        kind: ScheduledTransactionKinds::Transaction,
        value: 1_000,
        description: Some("Rent".into()),
        created_date: dt("2024-01-01T00:00:00"),
        account_id: Some(wallet.id),
        category_id: Some(bills.id),
        origin_account_id: None,
        destination_account_id: None,
        repeat: true,
        repeat_freq: Some(RepeatFrequencies::Months),
        repeat_interval: Some(1),
        infinite_repeat: Some(false),
        end_after_repeats: Some(3),
        current_repeat_count: None,
        next_date: None,
    };
    let new = build_new_scheduled(&pool, &body)
        .await
        .unwrap()
        .expect("valid repeating schedule");
    let inserted = db::scheduled_transactions::insert(&pool, &new).await.unwrap();
    assert_eq!(inserted.current_repeat_count, Some(0));

    let pay = PostScheduledTransactionPay {
        value: 1_000,
        description: "Rent payment".into(),
        date: dt("2024-01-01T10:00:00"),
        category_id: Some(bills.id),
        account_id: Some(wallet.id),
        origin_account_id: None,
        destination_account_id: None,
    };

    // First payment: schedule survives, count -> 1, next date advances one month.
    let advanced = pay_scheduled_impl(&pool, inserted.id, &pay).await.unwrap();
    assert_eq!(advanced.current_repeat_count, Some(1), "repeat count advanced");
    assert_eq!(
        advanced.next_date,
        Some(dt("2024-02-01T00:00:00")),
        "next occurrence is one month on"
    );
    assert_eq!(
        db::accounts::balance(&pool, wallet.id).await.unwrap(),
        -1_000,
        "the paid occurrence recorded a 1000 expense"
    );

    // Second and third payments; the third reaches end_after_repeats and removes the schedule.
    pay_scheduled_impl(&pool, inserted.id, &pay).await.unwrap();
    pay_scheduled_impl(&pool, inserted.id, &pay).await.unwrap();

    assert!(
        matches!(
            db::scheduled_transactions::get(&pool, inserted.id).await,
            Err(AppError::NotFound)
        ),
        "the schedule is removed after its final occurrence"
    );
    assert_eq!(
        db::accounts::balance(&pool, wallet.id).await.unwrap(),
        -3_000,
        "three occurrences recorded three expenses"
    );
}

/// Paying a transfer-kind schedule must materialise a real transfer (moving money between the
/// two accounts) and, for a one-off, remove the schedule.
#[tokio::test]
async fn paying_a_transfer_schedule_moves_money_and_removes_it() {
    let pool = fresh_pool().await;
    let a = db::accounts::insert(&pool, "A").await.unwrap();
    let b = db::accounts::insert(&pool, "B").await.unwrap();

    // Seed account A with a 10000 income so the transfer has funds to move.
    let seed = db::categories::insert(&pool, CategoryTypes::Income, "Seed")
        .await
        .unwrap();
    db::transactions::insert(
        &pool,
        &NewTransactionData {
            value: 10_000,
            description: "seed".into(),
            date: dt("2024-01-01T00:00:00"),
            account: a.id,
            category: seed.id,
        },
    )
    .await
    .unwrap();

    let body = PostScheduledTransaction {
        kind: ScheduledTransactionKinds::Transfer,
        value: 4_000,
        description: Some("Move to B".into()),
        created_date: dt("2024-01-02T00:00:00"),
        account_id: None,
        category_id: None,
        origin_account_id: Some(a.id),
        destination_account_id: Some(b.id),
        repeat: false,
        repeat_freq: None,
        repeat_interval: None,
        infinite_repeat: None,
        end_after_repeats: None,
        current_repeat_count: None,
        next_date: None,
    };
    let new = build_new_scheduled(&pool, &body)
        .await
        .unwrap()
        .expect("valid transfer schedule");
    let inserted = db::scheduled_transactions::insert(&pool, &new).await.unwrap();

    let pay = PostScheduledTransactionPay {
        value: 4_000,
        description: "Moved to B".into(),
        date: dt("2024-01-02T10:00:00"),
        category_id: None,
        account_id: None,
        origin_account_id: Some(a.id),
        destination_account_id: Some(b.id),
    };
    pay_scheduled_impl(&pool, inserted.id, &pay).await.unwrap();

    assert_eq!(
        db::accounts::balance(&pool, a.id).await.unwrap(),
        6_000,
        "10000 seed - 4000 transferred out"
    );
    assert_eq!(
        db::accounts::balance(&pool, b.id).await.unwrap(),
        4_000,
        "4000 transferred in"
    );
    assert!(
        matches!(
            db::scheduled_transactions::get(&pool, inserted.id).await,
            Err(AppError::NotFound)
        ),
        "the one-off transfer schedule is removed after payment"
    );
}

/// A transaction-kind payment missing its account/category, or referencing an unknown one,
/// is rejected — the validation the pay command relies on before writing anything.
#[tokio::test]
async fn paying_with_missing_or_unknown_refs_is_rejected() {
    let pool = fresh_pool().await;
    let wallet = db::accounts::insert(&pool, "Wallet").await.unwrap();
    let cat = db::categories::insert(&pool, CategoryTypes::Expense, "Misc")
        .await
        .unwrap();

    let body = PostScheduledTransaction {
        kind: ScheduledTransactionKinds::Transaction,
        value: 500,
        description: Some("One-off".into()),
        created_date: dt("2024-03-01T00:00:00"),
        account_id: Some(wallet.id),
        category_id: Some(cat.id),
        origin_account_id: None,
        destination_account_id: None,
        repeat: false,
        repeat_freq: None,
        repeat_interval: None,
        infinite_repeat: None,
        end_after_repeats: None,
        current_repeat_count: None,
        next_date: None,
    };
    let new = build_new_scheduled(&pool, &body).await.unwrap().unwrap();
    let inserted = db::scheduled_transactions::insert(&pool, &new).await.unwrap();

    // Missing account/category on the payment body -> BadRequest.
    let missing = PostScheduledTransactionPay {
        value: 500,
        description: "no refs".into(),
        date: dt("2024-03-01T10:00:00"),
        category_id: None,
        account_id: None,
        origin_account_id: None,
        destination_account_id: None,
    };
    assert!(
        matches!(
            pay_scheduled_impl(&pool, inserted.id, &missing).await,
            Err(AppError::BadRequest)
        ),
        "a transaction payment with no account/category is a bad request"
    );

    // Unknown account id -> NotFound.
    let unknown = PostScheduledTransactionPay {
        value: 500,
        description: "bad account".into(),
        date: dt("2024-03-01T10:00:00"),
        category_id: Some(cat.id),
        account_id: Some(9_999),
        origin_account_id: None,
        destination_account_id: None,
    };
    assert!(
        matches!(
            pay_scheduled_impl(&pool, inserted.id, &unknown).await,
            Err(AppError::NotFound)
        ),
        "an unknown account maps to not found"
    );

    // The failed attempts wrote nothing and left the schedule intact.
    assert_eq!(
        db::accounts::balance(&pool, wallet.id).await.unwrap(),
        0,
        "no transaction was recorded by the rejected payments"
    );
    assert!(
        db::scheduled_transactions::get(&pool, inserted.id)
            .await
            .is_ok(),
        "the schedule survives rejected payments"
    );
}
