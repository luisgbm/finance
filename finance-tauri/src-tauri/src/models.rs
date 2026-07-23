use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
//
// IMPORTANT: two distinct string representations are intentionally preserved:
//   * JSON  (serde, default)        -> PascalCase  e.g. "Expense", "TransferIncome"
//   * SQLite (sqlx, snake_case TEXT) -> e.g. "expense", "transfer_income"
// The React frontend relies on the PascalCase JSON values, while the database stores
// the snake_case strings. Do not collapse these into one representation.
//
// NOTE (SQLite port): the original Postgres backend annotated these with
// `#[sqlx(type_name = "...")]` to bind to Postgres ENUM types. SQLite has no ENUM types,
// so `type_name` is dropped and the enums map to plain TEXT columns; `rename_all` still
// controls the stored string values, keeping them byte-compatible with the old data.
// ---------------------------------------------------------------------------

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
pub enum CategoryTypes {
    Expense,
    Income,
    TransferIncome,
    TransferExpense,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
pub enum ScheduledTransactionKinds {
    Transaction,
    Transfer,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
pub enum RepeatFrequencies {
    Days,
    Weeks,
    Months,
    Years,
}

// ---------------------------------------------------------------------------
// Database row models
// ---------------------------------------------------------------------------

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct AppUser {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub categorytype: CategoryTypes,
    pub name: String,
    pub user_id: i32,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub account: i32,
    pub category: i32,
    pub user_id: i32,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Transfer {
    pub id: i32,
    pub origin_account: i32,
    pub destination_account: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct ScheduledTransaction {
    pub id: i32,
    pub kind: ScheduledTransactionKinds,
    pub value: i32,
    pub description: Option<String>,
    pub created_date: NaiveDateTime,
    pub account_id: Option<i32>,
    pub category_id: Option<i32>,
    pub origin_account_id: Option<i32>,
    pub destination_account_id: Option<i32>,
    pub repeat: bool,
    pub repeat_freq: Option<RepeatFrequencies>,
    pub repeat_interval: Option<i32>,
    pub infinite_repeat: Option<bool>,
    pub end_after_repeats: Option<i32>,
    pub current_repeat_count: Option<i32>,
    pub next_date: Option<NaiveDateTime>,
    pub user_id: i32,
}

// ---------------------------------------------------------------------------
// Request DTOs (deserialized from JSON request bodies)
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAppUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostCategory {
    pub categorytype: CategoryTypes,
    pub name: String,
}

pub type PatchCategory = PostCategory;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTransaction {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub category: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchTransaction {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub account: i32,
    pub category: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTransfer {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchTransfer {
    pub origin_account: i32,
    pub destination_account: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostScheduledTransaction {
    pub kind: ScheduledTransactionKinds,
    pub value: i32,
    pub description: Option<String>,
    pub created_date: NaiveDateTime,
    pub account_id: Option<i32>,
    pub category_id: Option<i32>,
    pub origin_account_id: Option<i32>,
    pub destination_account_id: Option<i32>,
    pub repeat: bool,
    pub repeat_freq: Option<RepeatFrequencies>,
    pub repeat_interval: Option<i32>,
    pub infinite_repeat: Option<bool>,
    pub end_after_repeats: Option<i32>,
    pub current_repeat_count: Option<i32>,
    pub next_date: Option<NaiveDateTime>,
}

pub type PatchScheduledTransaction = PostScheduledTransaction;

/// Internal (non-serialized) value object used to insert/update a transaction row.
#[derive(Debug, Clone)]
pub struct NewTransactionData {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub account: i32,
    pub category: i32,
    pub user_id: i32,
}

/// Internal (non-serialized) value object used to insert/update a transfer row.
#[derive(Debug, Clone)]
pub struct NewTransferData {
    pub origin_account: i32,
    pub destination_account: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

/// Internal (non-serialized) value object used to insert/update a scheduled transaction row.
#[derive(Debug, Clone)]
pub struct NewScheduledTransaction {
    pub kind: ScheduledTransactionKinds,
    pub value: i32,
    pub description: Option<String>,
    pub created_date: NaiveDateTime,
    pub account_id: Option<i32>,
    pub category_id: Option<i32>,
    pub origin_account_id: Option<i32>,
    pub destination_account_id: Option<i32>,
    pub repeat: bool,
    pub repeat_freq: Option<RepeatFrequencies>,
    pub repeat_interval: Option<i32>,
    pub infinite_repeat: Option<bool>,
    pub end_after_repeats: Option<i32>,
    pub current_repeat_count: Option<i32>,
    pub next_date: Option<NaiveDateTime>,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostScheduledTransactionPay {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub category_id: Option<i32>,
    pub account_id: Option<i32>,
    pub origin_account_id: Option<i32>,
    pub destination_account_id: Option<i32>,
}

// ---------------------------------------------------------------------------
// Response DTOs (serialized to JSON response bodies)
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialData {
    pub token: String,
    pub accounts: Vec<GetAccount>,
    pub categories: Vec<Category>,
    pub scheduled_transactions: Vec<GetScheduledTransaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccount {
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionTransferJoined {
    pub id: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub category_id: Option<i32>,
    pub category_type: CategoryTypes,
    pub category_name: Option<String>,
    pub account_id: i32,
    pub account_name: String,
    pub user_id: i32,
    pub from_account_id: Option<i32>,
    pub from_account_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetScheduledTransaction {
    pub id: i32,
    pub kind: ScheduledTransactionKinds,
    pub value: i32,
    pub description: Option<String>,
    pub created_date: NaiveDateTime,
    pub account_id: Option<i32>,
    pub account_name: Option<String>,
    pub category_id: Option<i32>,
    pub category_type: Option<CategoryTypes>,
    pub category_name: Option<String>,
    pub origin_account_id: Option<i32>,
    pub origin_account_name: Option<String>,
    pub destination_account_id: Option<i32>,
    pub destination_account_name: Option<String>,
    pub repeat: bool,
    pub repeat_freq: Option<RepeatFrequencies>,
    pub repeat_interval: Option<i32>,
    pub infinite_repeat: Option<bool>,
    pub end_after_repeats: Option<i32>,
    pub current_repeat_count: Option<i32>,
    pub next_date: Option<NaiveDateTime>,
    pub user_id: i32,
}
