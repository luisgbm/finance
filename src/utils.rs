use crate::models::{Account, Category, Transaction, TransactionJoined};

pub fn create_transaction_join(tuple: &(Transaction, Category, Account)) -> TransactionJoined {
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
    }
}