use crate::db_scheduled_transactions::DatabaseScheduledTransactions;
use crate::models_routes::GetScheduledTransaction;
use crate::utils;

pub fn get_all_scheduled_transactions(user_id: i32) -> Option<Vec<GetScheduledTransaction>> {
    let scheduled_transactions = DatabaseScheduledTransactions::new().get_all_scheduled_transactions(user_id);

    let mut get_scheduled_transactions = Vec::new();

    for scheduled_transaction in &scheduled_transactions {
        match utils::create_scheduled_transaction_join(&scheduled_transaction) {
            Some(get_scheduled_transaction) => {
                get_scheduled_transactions.push(get_scheduled_transaction);
            }
            None => {
                return None;
            }
        }
    }

    get_scheduled_transactions.sort_by_key(|t| t.created_date);
    get_scheduled_transactions.reverse();

    Some(get_scheduled_transactions)
}