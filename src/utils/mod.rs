use chrono::{Duration, NaiveDateTime};
use chronoutil::RelativeDuration;

use crate::database::accounts::DatabaseAccounts;
use crate::database::categories::DatabaseCategories;
use crate::database::models::{Account, Category, CategoryTypes, RepeatFrequencies, ScheduledTransaction, ScheduledTransactionKinds, Transaction, Transfer};
use crate::database::transactions::DatabaseTransactions;
use crate::database::transfers::DatabaseTransfers;
use crate::routes::models::{GetScheduledTransaction, TransactionTransferJoined};

pub mod jwt;

pub fn calculate_next_date(initial_date: NaiveDateTime, repeat: bool, repeat_freq: RepeatFrequencies, repeat_interval: i32, current_repeat_count: i32) -> NaiveDateTime {
    if repeat == true {
        match repeat_freq {
            RepeatFrequencies::Days => {
                initial_date + RelativeDuration::days((current_repeat_count * repeat_interval) as i64)
            }
            RepeatFrequencies::Weeks => {
                initial_date + Duration::weeks((current_repeat_count * repeat_interval) as i64)
            }
            RepeatFrequencies::Months => {
                initial_date + RelativeDuration::months(current_repeat_count * repeat_interval)
            }
            RepeatFrequencies::Years => {
                initial_date + RelativeDuration::years(current_repeat_count * repeat_interval)
            }
        }
    } else {
        initial_date
    }
}

pub fn create_transaction_from_transfer(transfer: &Transfer, category_type: CategoryTypes) -> TransactionTransferJoined {
    let transfer_account_id = if category_type == CategoryTypes::Expense { transfer.origin_account } else { transfer.destination_account };

    let acc = DatabaseAccounts::new().get_account(transfer_account_id, transfer.user_id)
        .expect("Error getting account information");

    let from_acc = DatabaseAccounts::new().get_account(transfer.origin_account, transfer.user_id)
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

pub fn create_scheduled_transaction_join(scheduled_transaction: &ScheduledTransaction) -> Option<GetScheduledTransaction> {
    let mut get_scheduled_transaction = GetScheduledTransaction {
        id: scheduled_transaction.id,
        kind: scheduled_transaction.kind,
        value: scheduled_transaction.value,
        description: scheduled_transaction.description.clone(),
        created_date: scheduled_transaction.created_date.clone(),
        account_id: None,
        account_name: None,
        category_id: None,
        category_type: None,
        category_name: None,
        origin_account_id: None,
        origin_account_name: None,
        destination_account_id: None,
        destination_account_name: None,
        repeat: scheduled_transaction.repeat,
        repeat_freq: scheduled_transaction.repeat_freq,
        repeat_interval: scheduled_transaction.repeat_interval,
        infinite_repeat: scheduled_transaction.infinite_repeat,
        end_after_repeats: scheduled_transaction.end_after_repeats,
        current_repeat_count: scheduled_transaction.current_repeat_count,
        next_date: scheduled_transaction.next_date,
        user_id: scheduled_transaction.user_id,
    };

    match scheduled_transaction.kind {
        ScheduledTransactionKinds::Transaction => {
            if scheduled_transaction.account_id.is_none() {
                return None;
            }

            if scheduled_transaction.category_id.is_none() {
                return None;
            }

            let account = DatabaseAccounts::new().get_account(scheduled_transaction.account_id.unwrap(), scheduled_transaction.user_id);

            if let Err(_) = account {
                return None;
            }

            let account = account.unwrap();

            let category = DatabaseCategories::new().get_category(scheduled_transaction.category_id.unwrap(), scheduled_transaction.user_id);

            if let Err(_) = category {
                return None;
            }

            let category = category.unwrap();

            get_scheduled_transaction.account_id = Some(account.id);
            get_scheduled_transaction.account_name = Some(account.name.clone());
            get_scheduled_transaction.category_id = Some(category.id);
            get_scheduled_transaction.category_type = Some(category.categorytype);
            get_scheduled_transaction.category_name = Some(category.name.clone());
        }
        ScheduledTransactionKinds::Transfer => {
            if scheduled_transaction.origin_account_id.is_none() {
                return None;
            }

            if scheduled_transaction.destination_account_id.is_none() {
                return None;
            }

            let origin_account = DatabaseAccounts::new().get_account(scheduled_transaction.origin_account_id.unwrap(), scheduled_transaction.user_id);

            if let Err(_) = origin_account {
                return None;
            }

            let origin_account = origin_account.unwrap();

            let destination_account = DatabaseAccounts::new().get_account(scheduled_transaction.destination_account_id.unwrap(), scheduled_transaction.user_id);

            if let Err(_) = destination_account {
                return None;
            }

            let destination_account = destination_account.unwrap();

            get_scheduled_transaction.origin_account_id = Some(origin_account.id);
            get_scheduled_transaction.origin_account_name = Some(origin_account.name.clone());
            get_scheduled_transaction.destination_account_id = Some(destination_account.id);
            get_scheduled_transaction.destination_account_name = Some(destination_account.name.clone());
        }
    }

    Some(get_scheduled_transaction)
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

    let transactions = DatabaseTransactions::new().get_all_transactions_of_account_joined(account_id, user_id);

    for transaction_tuple in &transactions {
        let transaction = &transaction_tuple.0;
        let category = &transaction_tuple.1;

        if category.categorytype == CategoryTypes::Income {
            balance += transaction.value;
        } else if category.categorytype == CategoryTypes::Expense {
            balance -= transaction.value;
        }
    }

    let transfers_from = DatabaseTransfers::new().get_transfers_from_account(account_id, user_id);

    for transfer_from in &transfers_from {
        balance -= transfer_from.value;
    }

    let transfers_to = DatabaseTransfers::new().get_transfers_to_account(account_id, user_id);

    for transfer_to in &transfers_to {
        balance += transfer_to.value;
    }

    balance
}
