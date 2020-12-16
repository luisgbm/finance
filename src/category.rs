use serde::{Serialize};

#[derive(Serialize, Debug)]
pub enum CategoryType {
    Expense,
    Income
}

#[derive(Serialize, Debug)]
pub struct Category {
    pub category_type: CategoryType,
    pub name: String
}

impl Category {
    pub fn new(category_type: CategoryType, name: &str) -> Category {
        Category {
            category_type,
            name: name.to_string()
        }
    }
}