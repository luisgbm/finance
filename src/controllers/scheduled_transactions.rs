use diesel::PgConnection;

use crate::routes::models::GetScheduledTransaction;
use crate::utils;

pub fn get_all_scheduled_transactions(user_id: i32, connection: &PgConnection) -> Option<Vec<GetScheduledTransaction>> {
    let scheduled_transactions = crate::database::scheduled_transactions::get_all_scheduled_transactions(user_id, connection);

    let mut get_scheduled_transactions = Vec::new();

    for scheduled_transaction in &scheduled_transactions {
        match utils::create_scheduled_transaction_join(&scheduled_transaction, connection) {
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