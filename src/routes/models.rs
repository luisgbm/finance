use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::database::models::{Category, CategoryTypes, RepeatFrequencies, ScheduledTransactionKinds};

#[derive(Serialize, Deserialize)]
pub struct InitialData {
    pub token: String,
    pub accounts: Vec<GetAccount>,
    pub categories: Vec<Category>,
    pub scheduled_transactions: Vec<GetScheduledTransaction>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct PostScheduledTransactionPay {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub category_id: Option<i32>,
    pub account_id: Option<i32>,
    pub origin_account_id: Option<i32>,
    pub destination_account_id: Option<i32>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct PostCategory {
    pub categorytype: CategoryTypes,
    pub name: String,
}

pub type PatchCategory = PostCategory;

#[derive(Serialize, Deserialize)]
pub struct GetAccount {
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PostAccount {
    pub name: String
}

pub type PatchAccount = PostAccount;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct PatchTransaction {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub account: i32,
    pub category: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PostTransaction {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub category: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PostTransfer {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct PatchTransfer {
    pub origin_account: i32,
    pub destination_account: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
}