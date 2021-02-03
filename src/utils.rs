use crate::finance_db::FinanceDB;
use crate::models::{Account, Category, CategoryTypes, Transaction, TransactionTransferJoined, Transfer};

pub fn create_transaction_from_transfer(transfer: &Transfer, category_type: CategoryTypes) -> TransactionTransferJoined {
    let transfer_account_id = if category_type == CategoryTypes::Expense { transfer.origin_account } else { transfer.destination_account };

    let acc = FinanceDB::new().get_account(transfer_account_id, transfer.user_id)
        .expect("Error getting account information");

    let from_acc = FinanceDB::new().get_account(transfer.origin_account, transfer.user_id)
        .expect("Error getting origin account information");

    let transaction = TransactionTransferJoined {
        id: transfer.id,
        value: transfer.value,
        description: transfer.description.clone(),
        date: transfer.date,
        category_id: None,
        category_type,
        category_name: None,
        account_id: transfer_account_id,
        account_name: acc.name,
        user_id: transfer.user_id,
        from_account_id: Some(from_acc.id),
        from_account_name: Some(from_acc.name),
    };

    transaction
}

pub fn create_transaction_join(tuple: &(Transaction, Category, Account), user_id: i32) -> TransactionTransferJoined {
    let transaction = &tuple.0;
    let category = &tuple.1;
    let account = &tuple.2;

    TransactionTransferJoined {
        id: transaction.id,
        value: transaction.value,
        description: transaction.description.clone(),
        date: transaction.date,
        category_id: Some(transaction.category),
        category_type: category.categorytype,
        category_name: Some(category.name.clone()),
        account_id: transaction.account,
        account_name: account.name.clone(),
        user_id,
        from_account_id: None,
        from_account_name: None,
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

    let transfers_from = FinanceDB::new().get_transfers_from_account(account_id, user_id);

    for transfer_from in &transfers_from {
        balance -= transfer_from.value;
    }

    let transfers_to = FinanceDB::new().get_transfers_to_account(account_id, user_id);

    for transfer_to in &transfers_to {
        balance += transfer_to.value;
    }

    balance
}