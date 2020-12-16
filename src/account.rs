use crate::transaction::Transaction;
use crate::category::CategoryType;

use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Account<'x> {
    pub name: String,
    pub transactions: Vec<&'x Transaction<'x>>
}

impl<'x> Account<'x> {
    pub fn new(name: &str) -> Account {
        Account {
            name: name.to_string(),
            transactions: Vec::new()
        }
    }

    pub fn add_transaction(&mut self, transaction: &'x Transaction) {
        self.transactions.push(transaction);
    }

    pub fn balance(&self) -> i64 {
        let mut total: i64 = 0;

        for transaction in &self.transactions {
            match transaction.category.category_type {
                CategoryType::Expense => total -= transaction.value,
                CategoryType::Income => total += transaction.value
            }
        }

        total
    }
}