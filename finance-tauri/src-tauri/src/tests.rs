//! End-to-end integration test for the embedded SQLite backend, exercised directly through
//! the `db` / `service` / command-helper layers (there is no HTTP server any more).
//!
//! It drives the full original flow against a throwaway SQLite database — register -> auth ->
//! accounts -> categories -> transactions -> balance -> transfers -> scheduled transaction ->
//! pay -> foreign-key cascade — validating the whole Postgres->SQLite port (placeholders,
//! bcrypt auth, enum-as-TEXT, the shared transaction/transfer id sequence, balance
//! computation and FK cascades) plus the scheduled-transaction build/pay logic that the
//! Tauri commands delegate to.

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::NaiveDateTime;
use sqlx::SqlitePool;

use crate::bootstrap;
use crate::commands::{build_new_scheduled, pay_scheduled_impl};
use crate::db;
use crate::error::AppError;
use crate::models::{
    CategoryTypes, NewTransactionData, NewTransferData, PostScheduledTransaction,
    PostScheduledTransactionPay, ScheduledTransactionKinds,
};
use crate::service;

/// Open a fresh, isolated database in the OS temp dir.
async fn fresh_pool() -> SqlitePool {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_path = std::env::temp_dir().join(format!("finance-tauri-test-{nanos}.db"));
    bootstrap::init(&db_path).await.expect("failed to init db")
}

fn dt(value: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S").unwrap()
}

#[tokio::test]
async fn full_flow() {
    let pool = fresh_pool().await;
    let bf_rounds = 4; // minimum bcrypt cost keeps the test fast

    // ----- register + duplicate handling ------------------------------------------------
    let user = db::users::insert(&pool, "tester", "s3cret", bf_rounds)
        .await
        .expect("register should succeed");
    let duplicate = db::users::insert(&pool, "tester", "whatever", bf_rounds).await;
    assert!(
        matches!(duplicate, Err(AppError::Conflict)),
        "duplicate user name maps the UNIQUE violation to Conflict"
    );
    let user_id = user.id;

    // ----- authentication ---------------------------------------------------------------
    assert!(
        db::users::authenticate(&pool, "tester", "s3cret")
            .await
            .unwrap()
            .is_some(),
        "correct password authenticates"
    );
    assert!(
        db::users::authenticate(&pool, "tester", "wrong")
            .await
            .unwrap()
            .is_none(),
        "wrong password does not authenticate"
    );

    // ----- initial payload --------------------------------------------------------------
    let initial = service::build_initial_data(&pool, user_id).await.unwrap();
    assert_eq!(initial.token, user_id.to_string(), "token carries the user id");
    assert!(initial.accounts.is_empty());

    // ----- accounts ---------------------------------------------------------------------
    let checking = db::accounts::insert(&pool, "Checking", user_id).await.unwrap();
    let savings = db::accounts::insert(&pool, "Savings", user_id).await.unwrap();

    // ----- categories (enum stored as TEXT) ---------------------------------------------
    let salary = db::categories::insert(&pool, CategoryTypes::Income, "Salary", user_id)
        .await
        .unwrap();
    let groceries = db::categories::insert(&pool, CategoryTypes::Expense, "Groceries", user_id)
        .await
        .unwrap();
    let income_cats = db::categories::get_all_by_type(&pool, CategoryTypes::Income, user_id)
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
            user_id,
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
            user_id,
        },
    )
    .await
    .unwrap();

    assert_eq!(
        db::accounts::balance(&pool, checking.id, user_id).await.unwrap(),
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
            user_id,
        },
    )
    .await
    .unwrap();
    assert_ne!(transfer.id, income_tx.id, "transfer/transaction ids must not collide");

    assert_eq!(db::accounts::balance(&pool, checking.id, user_id).await.unwrap(), 92_000);
    assert_eq!(db::accounts::balance(&pool, savings.id, user_id).await.unwrap(), 5_000);

    // Merged account view: 2 transactions + 1 outgoing transfer, all with distinct ids.
    let mut list = Vec::new();
    for row in db::transactions::get_all_of_account_joined(&pool, checking.id, user_id)
        .await
        .unwrap()
    {
        list.push(service::tx_join_to_dto(row));
    }
    for row in db::transfers::get_from_account_joined(&pool, checking.id, user_id)
        .await
        .unwrap()
    {
        list.push(service::transfer_from_to_dto(row, user_id));
    }
    for row in db::transfers::get_to_account_joined(&pool, checking.id, user_id)
        .await
        .unwrap()
    {
        list.push(service::transfer_to_to_dto(row, user_id));
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
    let new = build_new_scheduled(&pool, user_id, &body)
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
    pay_scheduled_impl(&pool, user_id, inserted.id, &pay).await.unwrap();

    assert!(
        matches!(
            db::scheduled_transactions::get(&pool, inserted.id, user_id).await,
            Err(AppError::NotFound)
        ),
        "paid one-off schedule is removed"
    );
    assert_eq!(
        db::accounts::balance(&pool, checking.id, user_id).await.unwrap(),
        90_000,
        "balance after pay"
    );

    // ----- foreign-key cascade ----------------------------------------------------------
    db::accounts::delete(&pool, savings.id, user_id).await.unwrap();
    assert!(
        matches!(
            db::accounts::get(&pool, savings.id, user_id).await,
            Err(AppError::NotFound)
        ),
        "deleted account is gone"
    );
}
