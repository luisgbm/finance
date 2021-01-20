use chrono::NaiveDateTime;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::schema::accounts;
use crate::schema::categories;
use crate::schema::transactions;
use crate::schema::users;

#[derive(DbEnum, Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum CategoryTypes {
    Expense,
    Income
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub categorytype: CategoryTypes,
    pub name: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub categorytype: CategoryTypes,
    pub name: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountWithBalance {
    pub id: i32,
    pub name: String,
    pub balance: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub account: i32,
    pub category: i32
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct TransactionJoined {
    pub id: i32,
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub category_id: i32,
    pub category_type: CategoryTypes,
    pub category_name: String,
    pub account_id: i32,
    pub account_name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="transactions"]
pub struct NewTransaction<'a> {
    pub value: i32,
    pub description: &'a str,
    pub date: NaiveDateTime,
    pub account: i32,
    pub category: i32
}

#[derive(Serialize, Deserialize)]
pub struct TransactionNoAccount {
    pub value: i32,
    pub description: String,
    pub date: NaiveDateTime,
    pub category: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub password: &'a str,
}