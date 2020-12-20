use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum CategoryType {
    Expense,
    Income
}

impl CategoryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CategoryType::Expense => "Expense",
            CategoryType::Income => "Income"
        }
    }

    pub fn from_str(s: &str) -> Option<CategoryType> {
        match s {
            "Expense" => Some(CategoryType::Expense),
            "Income" => Some(CategoryType::Income),
            _ => None
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Category {
    pub category_type: CategoryType,
    pub name: String,
    pub id: String
}

impl Category {
    pub fn new(category_type: CategoryType, name: &str) -> Category {
        Category {
            category_type,
            name: name.to_string(),
            id: String::new()
        }
    }

    pub fn new_with_id(category_type: CategoryType, name: &str, id: &str) -> Category {
        Category {
            category_type,
            name: name.to_string(),
            id: id.to_string()
        }
    }
}