use diesel::prelude::*;

use crate::db_finance::FinanceDB;
use crate::models_db::{AppUser, NewScheduledTransfer, ScheduledTransfer};

pub struct DatabaseScheduledTransfers {
    connection: FinanceDB
}

impl DatabaseScheduledTransfers {
    pub fn new() -> DatabaseScheduledTransfers {
        DatabaseScheduledTransfers {
            connection: FinanceDB::new()
        }
    }

    pub fn new_scheduled_transfer(&self, new_scheduled_transfer: &NewScheduledTransfer) -> ScheduledTransfer {
        use crate::schema::scheduled_transfers;

        diesel::insert_into(scheduled_transfers::table)
            .values(new_scheduled_transfer)
            .get_result(&self.connection.db_connection)
            .expect("Error saving new scheduled transfer")
    }

    pub fn get_all_scheduled_transfers(&self, app_user_id: i32) -> Vec<(ScheduledTransfer, AppUser)> {
        use crate::schema::scheduled_transfers::dsl::*;
        use crate::schema::scheduled_transfers;
        use crate::schema::app_users;

        scheduled_transfers::table.inner_join(app_users::table)
            .filter(user_id.eq(app_user_id))
            .order(created_date.asc())
            .load::<(ScheduledTransfer, AppUser)>(&self.connection.db_connection)
            .expect("Error loading scheduled transfers")
    }

    pub fn get_scheduled_transfer(&self, find_id: i32, app_user_id: i32) -> QueryResult<(ScheduledTransfer, AppUser)> {
        use crate::schema::scheduled_transfers::dsl::*;
        use crate::schema::scheduled_transfers;
        use crate::schema::app_users;

        scheduled_transfers::table.inner_join(app_users::table)
            .filter(user_id.eq(app_user_id))
            .filter(id.eq(find_id))
            .first::<(ScheduledTransfer, AppUser)>(&self.connection.db_connection)
    }

    pub fn update_scheduled_transfer(&self, update_id: i32, update_scheduled_transfer: &NewScheduledTransfer, app_user_id: i32) -> QueryResult<ScheduledTransfer> {
        use crate::schema::scheduled_transfers::dsl::*;

        diesel::update(scheduled_transfers.filter(user_id.eq(app_user_id)).find(update_id))
            .set((
                origin_account_id.eq(update_scheduled_transfer.origin_account_id),
                destination_account_id.eq(update_scheduled_transfer.destination_account_id),
                value.eq(update_scheduled_transfer.value),
                description.eq(update_scheduled_transfer.description),
                created_date.eq(update_scheduled_transfer.created_date),
                repeat.eq(update_scheduled_transfer.repeat),
                repeat_freq.eq(update_scheduled_transfer.repeat_freq),
                repeat_interval.eq(update_scheduled_transfer.repeat_interval),
                infinite_repeat.eq(update_scheduled_transfer.infinite_repeat),
                end_after_repeats.eq(update_scheduled_transfer.end_after_repeats),
                current_repeat_count.eq(update_scheduled_transfer.current_repeat_count),
                next_date.eq(update_scheduled_transfer.next_date)
            ))
            .get_result::<ScheduledTransfer>(&self.connection.db_connection)
    }

    pub fn delete_scheduled_transfer(&self, delete_id: i32, app_user_id: i32) -> QueryResult<ScheduledTransfer> {
        use crate::schema::scheduled_transfers::dsl::*;

        diesel::delete(scheduled_transfers.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<ScheduledTransfer>(&self.connection.db_connection)
    }
}