use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use chronoutil::RelativeDuration;
use diesel::result::Error;

use crate::db_accounts::DatabaseAccounts;
use crate::db_transactions::DatabaseTransactions;
use crate::db_transfers::DatabaseTransfers;
use crate::models_db::{Account, AppUser, Category, CategoryTypes, RepeatFrequencies, ScheduledTransaction, ScheduledTransfer, Transaction, Transfer};
use crate::models_routes::{GetScheduledTransaction, GetScheduledTransfer, ScheduledTransactionTransferJoined, ScheduledTransactionType, TransactionTransferJoined};

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

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days()
}

pub fn add_days_to_naive_date_time(days: i32, naive: &NaiveDateTime) -> NaiveDateTime {
    naive.clone() + Duration::days(days as i64)
}

pub fn add_years_to_naive_date_time(years: i32, naive: &NaiveDateTime) -> NaiveDateTime {
    let mut new_date = naive.clone();

    for _i in 0..years {
        for _j in 0..12 {
            new_date = add_one_month_to_naive_date_time(&new_date);
        }
    }

    new_date
}

pub fn add_months_to_naive_date_time(months: i32, naive: &NaiveDateTime) -> NaiveDateTime {
    let mut new_date = naive.clone();

    for _i in 0..months {
        new_date = add_one_month_to_naive_date_time(&new_date);
    }

    new_date
}

pub fn add_one_month_to_naive_date_time(naive: &NaiveDateTime) -> NaiveDateTime {
    let y = naive.date().year();
    let m = naive.date().month();

    let days_to_add = get_days_from_month(
        match y {
            12 => y + 1,
            _ => y
        },
        match m {
            12 => 1,
            _ => m + 1,
        },
    );

    add_days_to_naive_date_time(days_to_add as i32, naive)
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

pub fn create_scheduled_transfer_join(tuple: &(ScheduledTransfer, AppUser)) -> Result<GetScheduledTransfer, Error> {
    let scheduled_transfer = &tuple.0;
    let app_user = &tuple.1;

    match DatabaseAccounts::new().get_account(scheduled_transfer.origin_account_id, app_user.id) {
        Ok(origin_account) => {
            match DatabaseAccounts::new().get_account(scheduled_transfer.destination_account_id, app_user.id) {
                Ok(destination_account) => {
                    Ok(GetScheduledTransfer {
                        id: scheduled_transfer.id,
                        origin_account_id: origin_account.id,
                        origin_account_name: origin_account.name.clone(),
                        destination_account_id: destination_account.id,
                        destination_account_name: destination_account.name.clone(),
                        value: scheduled_transfer.value,
                        description: scheduled_transfer.description.clone(),
                        created_date: scheduled_transfer.created_date.clone(),
                        repeat: scheduled_transfer.repeat,
                        repeat_freq: scheduled_transfer.repeat_freq,
                        repeat_interval: scheduled_transfer.repeat_interval,
                        infinite_repeat: scheduled_transfer.infinite_repeat,
                        end_after_repeats: scheduled_transfer.end_after_repeats,
                        current_repeat_count: scheduled_transfer.current_repeat_count,
                        next_date: scheduled_transfer.next_date,
                        user_id: app_user.id,
                    })
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}

pub fn get_scheduled_transaction_to_join(get_scheduled_transaction: &GetScheduledTransaction) -> ScheduledTransactionTransferJoined {
    ScheduledTransactionTransferJoined {
        id: get_scheduled_transaction.id,
        scheduled_type: ScheduledTransactionType::Transaction,
        value: get_scheduled_transaction.value,
        description: get_scheduled_transaction.description.clone(),
        created_date: get_scheduled_transaction.created_date.clone(),
        account_id: Some(get_scheduled_transaction.account_id),
        account_name: Some(get_scheduled_transaction.account_name.clone()),
        origin_account_id: None,
        origin_account_name: None,
        destination_account_id: None,
        destination_account_name: None,
        repeat: get_scheduled_transaction.repeat,
        repeat_freq: get_scheduled_transaction.repeat_freq,
        repeat_interval: get_scheduled_transaction.repeat_interval,
        infinite_repeat: get_scheduled_transaction.infinite_repeat,
        end_after_repeats: get_scheduled_transaction.end_after_repeats,
        current_repeat_count: get_scheduled_transaction.current_repeat_count,
        next_date: get_scheduled_transaction.next_date,
        user_id: get_scheduled_transaction.user_id,
    }
}

pub fn get_scheduled_transfer_to_join(get_scheduled_transfer: &GetScheduledTransfer) -> ScheduledTransactionTransferJoined {
    ScheduledTransactionTransferJoined {
        id: get_scheduled_transfer.id,
        scheduled_type: ScheduledTransactionType::Transfer,
        value: get_scheduled_transfer.value,
        description: get_scheduled_transfer.description.clone(),
        created_date: get_scheduled_transfer.created_date.clone(),
        account_id: None,
        account_name: None,
        origin_account_id: Some(get_scheduled_transfer.origin_account_id),
        origin_account_name: Some(get_scheduled_transfer.origin_account_name.clone()),
        destination_account_id: Some(get_scheduled_transfer.destination_account_id),
        destination_account_name: Some(get_scheduled_transfer.destination_account_name.clone()),
        repeat: get_scheduled_transfer.repeat,
        repeat_freq: get_scheduled_transfer.repeat_freq,
        repeat_interval: get_scheduled_transfer.repeat_interval,
        infinite_repeat: get_scheduled_transfer.infinite_repeat,
        end_after_repeats: get_scheduled_transfer.end_after_repeats,
        current_repeat_count: get_scheduled_transfer.current_repeat_count,
        next_date: get_scheduled_transfer.next_date,
        user_id: get_scheduled_transfer.user_id,
    }
}

pub fn create_scheduled_transaction_join(tuple: &(ScheduledTransaction, Category, Account, AppUser)) -> GetScheduledTransaction {
    let scheduled_transaction = &tuple.0;
    let category = &tuple.1;
    let account = &tuple.2;
    let app_user = &tuple.3;

    GetScheduledTransaction {
        id: scheduled_transaction.id,
        account_id: account.id,
        account_name: account.name.clone(),
        value: scheduled_transaction.value,
        description: scheduled_transaction.description.clone(),
        category_id: category.id,
        category_type: category.categorytype,
        category_name: category.name.clone(),
        created_date: scheduled_transaction.created_date.clone(),
        repeat: scheduled_transaction.repeat,
        repeat_freq: scheduled_transaction.repeat_freq,
        repeat_interval: scheduled_transaction.repeat_interval,
        infinite_repeat: scheduled_transaction.infinite_repeat,
        end_after_repeats: scheduled_transaction.end_after_repeats,
        current_repeat_count: scheduled_transaction.current_repeat_count,
        next_date: scheduled_transaction.next_date.clone(),
        user_id: app_user.id,
    }
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