use crate::category::Category;

use chrono::{DateTime, Utc};
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Transaction<'x> {
    pub category: &'x Category,
    pub value: i64,
    pub timestamp: DateTime<Utc>
}

impl<'x> Transaction<'x> {
    pub fn new(category: &'x Category, value: i64, timestamp: DateTime<Utc>) -> Transaction {
        Transaction {
            category,
            value,
            timestamp
        }
    }
}