use crate::db_scheduled_transactions::DatabaseScheduledTransactions;
use crate::db_scheduled_transfers::DatabaseScheduledTransfers;
use crate::models_routes::ScheduledTransactionTransferJoined;
use crate::utils;

pub fn get_all_scheduled_transactions(user_id: i32) -> Option<Vec<ScheduledTransactionTransferJoined>> {
    let scheduled_transactions_tuples = DatabaseScheduledTransactions::new().get_all_scheduled_transactions(user_id);
    let scheduled_transfers_tuples = DatabaseScheduledTransfers::new().get_all_scheduled_transfers(user_id);

    let mut scheduled_transactions_transfers = Vec::new();

    for scheduled_transaction_tuple in &scheduled_transactions_tuples {
        let get_scheduled_transaction = utils::create_scheduled_transaction_join(scheduled_transaction_tuple);
        scheduled_transactions_transfers.push(utils::get_scheduled_transaction_to_join(&get_scheduled_transaction));
    }

    for scheduled_transfer_tuple in &scheduled_transfers_tuples {
        match utils::create_scheduled_transfer_join(scheduled_transfer_tuple) {
            Ok(scheduled_transfer) => {
                let get_scheduled_transfer = utils::get_scheduled_transfer_to_join(&scheduled_transfer);
                scheduled_transactions_transfers.push(get_scheduled_transfer);
            }
            Err(_) => {
                return None;
            }
        }
    }

    scheduled_transactions_transfers.sort_by_key(|t| t.created_date);
    scheduled_transactions_transfers.reverse();

    Some(scheduled_transactions_transfers)
}