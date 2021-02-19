use chrono::NaiveDateTime;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::schema::accounts;
use crate::schema::app_users;
use crate::schema::categories;
use crate::schema::scheduled_transactions;
use crate::schema::transactions;
use crate::schema::transfers;

#[derive(DbEnum, Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum CategoryTypes {
    Expense,
    Income,
    TransferIncome,
    TransferExpense,
}

#[derive(DbEnum, Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum RepeatFrequencies {
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ScheduledTransfer {
    pub id: i32,
    pub origin_account_id: i32,
    pub destination_account_id: i32,
    pub value: i32,
    pub description: String,
    pub created_date: NaiveDateTime,
    pub repeat: bool,
    pub repeat_freq: Option<RepeatFrequencies>,
    pub repeat_interval: Option<i32>,
    pub infinite_repeat: Option<bool>,
    pub end_after_repeats: Option<i32>,
    pub current_repeat_count: Option<i32>,
    pub next_date: Option<NaiveDateTime>,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "scheduled_transactions"]
pub struct NewScheduledTransfer<'a> {
    pub account_id: i32,
    pub value: i32,
    pub description: &'a str,
    pub category_id: i32,
    pub created_date: NaiveDateTime,
    pub repeat: bool,
    pub repeat_freq: Option<RepeatFrequencies>,
    pub repeat_interval: Option<i32>,
    pub infinite_repeat: Option<bool>,
    pub end_after_repeats: Option<i32>,
    pub current_repeat_count: Option<i32>,
    pub next_date: Option<NaiveDateTime>,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ScheduledTransaction {
    pub id: i32,
    pub account_id: i32,
    pub value: i32,
    pub description: String,
    pub category_id: i32,
    pub created_date: NaiveDateTime,
    pub repeat: bool,
    pub repeat_freq: Option<RepeatFrequencies>,
    pub repeat_interval: Option<i32>,
    pub infinite_repeat: Option<bool>,
    pub end_after_repeats: Option<i32>,
    pub current_repeat_count: Option<i32>,
    pub next_date: Option<NaiveDateTime>,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "scheduled_transactions"]
pub struct NewScheduledTransaction<'a> {
    pub account_id: i32,
    pub value: i32,
    pub description: &'a str,
    pub category_id: i32,
    pub created_date: NaiveDateTime,
    pub repeat: bool,
    pub repeat_freq: Option<RepeatFrequencies>,
    pub repeat_interval: Option<i32>,
    pub infinite_repeat: Option<bool>,
    pub end_after_repeats: Option<i32>,
    pub current_repeat_count: Option<i32>,
    pub next_date: Option<NaiveDateTime>,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub categorytype: CategoryTypes,
    pub name: String,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub categorytype: CategoryTypes,
    pub name: &'a str,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub account: i32,
    pub category: i32,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub value: i32,
    pub description: &'a str,
    pub date: NaiveDateTime,
    pub account: i32,
    pub category: i32,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct AppUser {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "app_users"]
pub struct NewAppUser<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Transfer {
    pub id: i32,
    pub origin_account: i32,
    pub destination_account: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "transfers"]
pub struct NewTransfer<'a> {
    pub origin_account: i32,
    pub destination_account: i32,
    pub value: i32,
    pub description: &'a str,
    pub date: NaiveDateTime,
    pub user_id: i32,
}