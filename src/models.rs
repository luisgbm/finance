use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel_derive_enum::DbEnum;

use crate::schema::categories;
use crate::schema::accounts;
use crate::schema::transactions;

#[derive(DbEnum, Debug, Serialize, Deserialize, Copy, Clone)]
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
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub categorytype: CategoryTypes,
    pub name: &'a str
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="accounts"]
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
    pub category: i32
}
