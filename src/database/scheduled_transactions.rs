use diesel::prelude::*;

use crate::database::finance::FinanceDB;
use crate::database::models::{NewScheduledTransaction, ScheduledTransaction};

pub struct DatabaseScheduledTransactions {
    connection: FinanceDB
}

impl DatabaseScheduledTransactions {
    pub fn new() -> DatabaseScheduledTransactions {
        DatabaseScheduledTransactions {
            connection: FinanceDB::new()
        }
    }

    pub fn new_scheduled_transaction(&self, new_scheduled_transaction: &NewScheduledTransaction) -> ScheduledTransaction {
        use crate::database::schema::scheduled_transactions;

        diesel::insert_into(scheduled_transactions::table)
            .values(new_scheduled_transaction)
            .get_result(&self.connection.db_connection)
            .expect("Error saving new scheduled transaction")
    }

    pub fn get_all_scheduled_transactions(&self, app_user_id: i32) -> Vec<ScheduledTransaction> {
        use crate::database::schema::scheduled_transactions::dsl::*;
        use crate::database::schema::scheduled_transactions;

        scheduled_transactions::table
            .filter(user_id.eq(app_user_id))
            .order(created_date.asc())
            .load::<ScheduledTransaction>(&self.connection.db_connection)
            .expect("Error loading scheduled transactions")
    }

    pub fn get_scheduled_transaction(&self, find_id: i32, app_user_id: i32) -> QueryResult<ScheduledTransaction> {
        use crate::database::schema::scheduled_transactions::dsl::*;
        use crate::database::schema::scheduled_transactions;

        scheduled_transactions::table
            .filter(user_id.eq(app_user_id))
            .filter(id.eq(find_id))
            .first::<ScheduledTransaction>(&self.connection.db_connection)
    }

    pub fn update_scheduled_transaction(&self, update_id: i32, update_scheduled_transaction: &NewScheduledTransaction, app_user_id: i32) -> QueryResult<ScheduledTransaction> {
        use crate::database::schema::scheduled_transactions::dsl::*;

        diesel::update(scheduled_transactions.filter(user_id.eq(app_user_id)).find(update_id))
            .set((
                kind.eq(update_scheduled_transaction.kind),
                value.eq(update_scheduled_transaction.value),
                description.eq(update_scheduled_transaction.description.as_ref()),
                created_date.eq(update_scheduled_transaction.created_date),
                account_id.eq(update_scheduled_transaction.account_id),
                category_id.eq(update_scheduled_transaction.category_id),
                origin_account_id.eq(update_scheduled_transaction.origin_account_id),
                destination_account_id.eq(update_scheduled_transaction.destination_account_id),
                repeat.eq(update_scheduled_transaction.repeat),
                repeat_freq.eq(update_scheduled_transaction.repeat_freq),
                repeat_interval.eq(update_scheduled_transaction.repeat_interval),
                infinite_repeat.eq(update_scheduled_transaction.infinite_repeat),
                end_after_repeats.eq(update_scheduled_transaction.end_after_repeats),
                current_repeat_count.eq(update_scheduled_transaction.current_repeat_count),
                next_date.eq(update_scheduled_transaction.next_date)
            ))
            .get_result::<ScheduledTransaction>(&self.connection.db_connection)
    }

    pub fn delete_scheduled_transaction(&self, delete_id: i32, app_user_id: i32) -> QueryResult<ScheduledTransaction> {
        use crate::database::schema::scheduled_transactions::dsl::*;

        diesel::delete(scheduled_transactions.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<ScheduledTransaction>(&self.connection.db_connection)
    }
}