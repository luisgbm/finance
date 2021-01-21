use crate::finance_db::FinanceDB;
use crate::models::{Account, Category, CategoryTypes, Transaction, TransactionJoined};

pub fn create_transaction_join(tuple: &(Transaction, Category, Account), user_id: i32) -> TransactionJoined {
    let transaction = &tuple.0;
    let category = &tuple.1;
    let account = &tuple.2;

    TransactionJoined {
        id: transaction.id,
        value: transaction.value,
        description: transaction.description.clone(),
        date: transaction.date,
        category_id: transaction.category,
        category_type: category.categorytype,
        category_name: category.name.clone(),
        account_id: transaction.account,
        account_name: account.name.clone(),
        user_id
    }
}

pub fn get_account_balance(account_id: i32, user_id: i32) -> i32 {
    let mut balance: i32 = 0;

    let transactions = FinanceDB::new().get_all_transactions_of_account_joined(account_id, user_id);

    for transaction_tuple in &transactions {
        let transaction = &transaction_tuple.0;
        let category = &transaction_tuple.1;

        if category.categorytype == CategoryTypes::Income {
            balance += transaction.value;
        } else if category.categorytype == CategoryTypes::Expense {
            balance -= transaction.value;
        }
    }

    balance
}