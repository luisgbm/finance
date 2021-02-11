use diesel::prelude::*;

use crate::db_finance::FinanceDB;
use crate::models_db::{Account, AppUser, Category, NewScheduledTransaction, ScheduledTransaction};

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
        use crate::schema::scheduled_transactions;

        diesel::insert_into(scheduled_transactions::table)
            .values(new_scheduled_transaction)
            .get_result(&self.connection.db_connection)
            .expect("Error saving new scheduled transaction")
    }

    pub fn get_all_scheduled_transactions(&self, app_user_id: i32) -> Vec<(ScheduledTransaction, Category, Account, AppUser)> {
        use crate::schema::scheduled_transactions::dsl::*;
        use crate::schema::scheduled_transactions;
        use crate::schema::categories;
        use crate::schema::accounts;
        use crate::schema::app_users;

        scheduled_transactions::table.inner_join(categories::table).inner_join(accounts::table).inner_join(app_users::table)
            .filter(user_id.eq(app_user_id))
            .order(created_date.asc())
            .load::<(ScheduledTransaction, Category, Account, AppUser)>(&self.connection.db_connection)
            .expect("Error loading scheduled transactions")
    }

    pub fn get_scheduled_transaction(&self, find_id: i32, app_user_id: i32) -> QueryResult<(ScheduledTransaction, Category, Account, AppUser)> {
        use crate::schema::scheduled_transactions::dsl::*;
        use crate::schema::scheduled_transactions;
        use crate::schema::categories;
        use crate::schema::accounts;
        use crate::schema::app_users;

        scheduled_transactions::table.inner_join(categories::table).inner_join(accounts::table).inner_join(app_users::table)
            .filter(user_id.eq(app_user_id))
            .filter(id.eq(find_id))
            .first::<(ScheduledTransaction, Category, Account, AppUser)>(&self.connection.db_connection)
    }

    pub fn update_scheduled_transaction(&self, update_id: i32, update_scheduled_transaction: &NewScheduledTransaction, app_user_id: i32) -> QueryResult<ScheduledTransaction> {
        use crate::schema::scheduled_transactions::dsl::*;

        diesel::update(scheduled_transactions.filter(user_id.eq(app_user_id)).find(update_id))
            .set((
                account_id.eq(update_scheduled_transaction.account_id),
                value.eq(update_scheduled_transaction.value),
                description.eq(update_scheduled_transaction.description),
                category_id.eq(update_scheduled_transaction.category_id),
                created_date.eq(update_scheduled_transaction.created_date),
                repeat.eq(update_scheduled_transaction.repeat),
                repeat_freq.eq(update_scheduled_transaction.repeat_freq),
                repeat_interval.eq(update_scheduled_transaction.repeat_interval),
                end_after_repeats.eq(update_scheduled_transaction.end_after_repeats),
                current_repeat_count.eq(update_scheduled_transaction.current_repeat_count),
                next_date.eq(update_scheduled_transaction.next_date)
            ))
            .get_result::<ScheduledTransaction>(&self.connection.db_connection)
    }

    pub fn delete_scheduled_transaction(&self, delete_id: i32, app_user_id: i32) -> QueryResult<ScheduledTransaction> {
        use crate::schema::scheduled_transactions::dsl::*;

        diesel::delete(scheduled_transactions.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<ScheduledTransaction>(&self.connection.db_connection)
    }
}