//! End-to-end integration test for the embedded SQLite-backed API.
//!
//! It boots the real Axum router (via `bootstrap::build_router`) against a throwaway
//! SQLite database and drives the full original REST flow over HTTP with `reqwest`:
//! register -> auth -> accounts -> categories -> transactions -> balance -> transfers ->
//! scheduled transaction -> pay. This validates the whole Postgres->SQLite port
//! (placeholders, bcrypt auth, enum-as-TEXT, the shared transaction/transfer id sequence,
//! balance computation and foreign-key cascades) in one pass.

use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::{Client, StatusCode};
use serde_json::{json, Value};

/// Boot the API on an ephemeral loopback port against a fresh temp database.
async fn spawn_server() -> SocketAddr {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_path = std::env::temp_dir().join(format!("finance-tauri-test-{nanos}.db"));

    let app = finance_tauri_lib::bootstrap::build_router(&db_path)
        .await
        .expect("failed to build router");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("failed to bind test listener");
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    addr
}

#[tokio::test]
async fn full_api_flow() {
    let addr = spawn_server().await;
    let base = format!("http://{addr}/api");
    let client = Client::new();

    // ----- unauthenticated access is rejected -------------------------------------------
    let res = client.get(format!("{base}/accounts")).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED, "no token => 401");

    // ----- register a user --------------------------------------------------------------
    let res = client
        .post(format!("{base}/users"))
        .json(&json!({ "name": "tester", "password": "s3cret" }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK, "register should succeed");
    let initial: Value = res.json().await.unwrap();
    let token = initial["token"].as_str().expect("token present").to_string();
    assert!(initial["accounts"].as_array().unwrap().is_empty());

    // Duplicate name must map the SQLite UNIQUE violation to 409 Conflict.
    let res = client
        .post(format!("{base}/users"))
        .json(&json!({ "name": "tester", "password": "whatever" }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CONFLICT, "duplicate user => 409");

    // ----- login + token validation -----------------------------------------------------
    let res = client
        .post(format!("{base}/login"))
        .json(&json!({ "name": "tester", "password": "s3cret" }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK, "login should succeed");

    let res = client
        .post(format!("{base}/login"))
        .json(&json!({ "name": "tester", "password": "wrong" }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED, "bad password => 401");

    let auth = |req: reqwest::RequestBuilder| req.bearer_auth(&token);

    let res = auth(client.get(format!("{base}/token"))).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK, "token refresh should succeed");

    // ----- accounts ---------------------------------------------------------------------
    let checking: Value = auth(client.post(format!("{base}/accounts")))
        .json(&json!({ "name": "Checking" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let checking_id = checking["id"].as_i64().unwrap();
    assert_eq!(checking["balance"].as_i64().unwrap(), 0);

    let savings: Value = auth(client.post(format!("{base}/accounts")))
        .json(&json!({ "name": "Savings" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let savings_id = savings["id"].as_i64().unwrap();

    // ----- categories (enum stored as TEXT) ---------------------------------------------
    let salary: Value = auth(client.post(format!("{base}/categories")))
        .json(&json!({ "categorytype": "Income", "name": "Salary" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let salary_id = salary["id"].as_i64().unwrap();
    assert_eq!(salary["categorytype"].as_str().unwrap(), "Income");

    let groceries: Value = auth(client.post(format!("{base}/categories")))
        .json(&json!({ "categorytype": "Expense", "name": "Groceries" }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let groceries_id = groceries["id"].as_i64().unwrap();

    // The typed category filters should return exactly the matching category.
    let income_cats: Value = auth(client.get(format!("{base}/categories/income")))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(income_cats.as_array().unwrap().len(), 1);

    // ----- transactions -----------------------------------------------------------------
    let income_tx: Value = auth(client.post(format!("{base}/transactions/account/{checking_id}")))
        .json(&json!({
            "value": 100_000,
            "description": "January salary",
            "date": "2024-01-05T09:00:00",
            "category": salary_id
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let income_tx_id = income_tx["id"].as_i64().unwrap();

    auth(client.post(format!("{base}/transactions/account/{checking_id}")))
        .json(&json!({
            "value": 3_000,
            "description": "Groceries",
            "date": "2024-01-06T18:30:00",
            "category": groceries_id
        }))
        .send()
        .await
        .unwrap();

    // income - expense = 100000 - 3000 = 97000
    let acc = get_account(&client, &base, &token, checking_id).await;
    assert_eq!(acc["balance"].as_i64().unwrap(), 97_000, "balance after transactions");

    // ----- transfer (shares the id sequence with transactions) --------------------------
    let transfer: Value = auth(client.post(format!(
        "{base}/transfers/from/{checking_id}/to/{savings_id}"
    )))
    .json(&json!({
        "value": 5_000,
        "description": "Move to savings",
        "date": "2024-01-07T12:00:00"
    }))
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap();
    let transfer_id = transfer["id"].as_i64().unwrap();

    // The transfer id must not collide with any transaction id (shared sequence).
    assert_ne!(transfer_id, income_tx_id, "transfer/transaction ids must not collide");

    // Checking: 97000 - 5000 = 92000; Savings: +5000
    let checking_acc = get_account(&client, &base, &token, checking_id).await;
    assert_eq!(checking_acc["balance"].as_i64().unwrap(), 92_000);
    let savings_acc = get_account(&client, &base, &token, savings_id).await;
    assert_eq!(savings_acc["balance"].as_i64().unwrap(), 5_000);

    // The merged account view returns the 2 transactions + 1 outgoing transfer, all with
    // distinct ids (the property the shared sequence guarantees for React keys).
    let joined: Value = auth(client.get(format!("{base}/transactions/account/{checking_id}")))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let rows = joined.as_array().unwrap();
    assert_eq!(rows.len(), 3, "2 transactions + 1 transfer");
    let mut ids: Vec<i64> = rows.iter().map(|r| r["id"].as_i64().unwrap()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 3, "all ids in the merged view are unique");

    // ----- scheduled transaction + pay --------------------------------------------------
    let scheduled: Value = auth(client.post(format!("{base}/scheduled-transactions")))
        .json(&json!({
            "kind": "Transaction",
            "value": 2_000,
            "description": "One-off scheduled expense",
            "created_date": "2024-01-10T00:00:00",
            "account_id": checking_id,
            "category_id": groceries_id,
            "repeat": false
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let scheduled_id = scheduled["id"].as_i64().unwrap();
    assert_eq!(scheduled["account_name"].as_str().unwrap(), "Checking");

    let res = auth(client.post(format!(
        "{base}/scheduled-transactions/{scheduled_id}/pay"
    )))
    .json(&json!({
        "value": 2_000,
        "description": "Paid scheduled expense",
        "date": "2024-01-10T10:00:00",
        "account_id": checking_id,
        "category_id": groceries_id
    }))
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), StatusCode::OK, "pay should succeed");

    // A non-repeating schedule is deleted after paying.
    let res = auth(client.get(format!("{base}/scheduled-transactions/{scheduled_id}")))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "paid one-off schedule removed");

    // Paying created a real 2000 expense: 92000 - 2000 = 90000.
    let checking_acc = get_account(&client, &base, &token, checking_id).await;
    assert_eq!(checking_acc["balance"].as_i64().unwrap(), 90_000, "balance after pay");

    // ----- foreign-key cascade ----------------------------------------------------------
    // Deleting the account should cascade-delete its transactions (PRAGMA foreign_keys=ON).
    let res = auth(client.delete(format!("{base}/accounts/{savings_id}")))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK, "delete account");

    let res = auth(client.get(format!("{base}/accounts/{savings_id}")))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "deleted account is gone");
}

async fn get_account(client: &Client, base: &str, token: &str, id: i64) -> Value {
    client
        .get(format!("{base}/accounts/{id}"))
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
