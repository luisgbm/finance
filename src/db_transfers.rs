use diesel::prelude::*;
use diesel::result::Error;

use crate::db_finance::FinanceDB;
use crate::models_db::{NewTransfer, Transfer};

pub struct DatabaseTransfers {
    connection: FinanceDB
}

impl DatabaseTransfers {
    pub fn new() -> DatabaseTransfers {
        DatabaseTransfers {
            connection: FinanceDB::new()
        }
    }

    pub fn new_transfer(&self, new_transfer: &NewTransfer) -> Transfer {
        use crate::schema::transfers;

        diesel::insert_into(transfers::table)
            .values(new_transfer)
            .get_result(&self.connection.db_connection)
            .expect("Error saving new transfer")
    }

    pub fn get_transfers_from_account(&self, from_account: i32, app_user_id: i32) -> Vec<Transfer> {
        use crate::schema::transfers::dsl::*;

        transfers
            .filter(user_id.eq(app_user_id))
            .filter(origin_account.eq(from_account))
            .load::<Transfer>(&self.connection.db_connection)
            .expect("Error loading transfers")
    }

    pub fn get_transfers_to_account(&self, to_account: i32, app_user_id: i32) -> Vec<Transfer> {
        use crate::schema::transfers::dsl::*;

        transfers
            .filter(user_id.eq(app_user_id))
            .filter(destination_account.eq(to_account))
            .load::<Transfer>(&self.connection.db_connection)
            .expect("Error loading transfers")
    }

    pub fn get_transfer(&self, find_id: i32, app_user_id: i32) -> Result<Transfer, Error> {
        use crate::schema::transfers::dsl::*;

        transfers
            .filter(user_id.eq(app_user_id))
            .filter(id.eq(find_id))
            .first::<Transfer>(&self.connection.db_connection)
    }

    pub fn update_transfer(&self, update_id: i32, update_transfer: &NewTransfer, app_user_id: i32) -> Result<Transfer, Error> {
        use crate::schema::transfers::dsl::*;

        diesel::update(transfers.filter(user_id.eq(app_user_id)).find(update_id))
            .set((
                origin_account.eq(update_transfer.origin_account),
                destination_account.eq(update_transfer.destination_account),
                value.eq(update_transfer.value),
                description.eq(update_transfer.description),
                date.eq(update_transfer.date)
            ))
            .get_result::<Transfer>(&self.connection.db_connection)
    }

    pub fn delete_transfer(&self, delete_id: i32, app_user_id: i32) -> Result<Transfer, Error> {
        use crate::schema::transfers::dsl::*;

        diesel::delete(transfers.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<Transfer>(&self.connection.db_connection)
    }
}