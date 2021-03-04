use diesel::prelude::*;
use diesel::result::Error;

use crate::database::models::{NewTransfer, Transfer};

pub fn new_transfer(new_transfer: &NewTransfer, connection: &PgConnection) -> Transfer {
    use crate::database::schema::transfers;

    diesel::insert_into(transfers::table)
        .values(new_transfer)
        .get_result(connection)
        .expect("Error saving new transfer")
}

pub fn get_transfers_from_account(from_account: i32, app_user_id: i32, connection: &PgConnection) -> Vec<Transfer> {
    use crate::database::schema::transfers::dsl::*;

    transfers
        .filter(user_id.eq(app_user_id))
        .filter(origin_account.eq(from_account))
        .load::<Transfer>(connection)
        .expect("Error loading transfers")
}

pub fn get_transfers_to_account(to_account: i32, app_user_id: i32, connection: &PgConnection) -> Vec<Transfer> {
    use crate::database::schema::transfers::dsl::*;

    transfers
        .filter(user_id.eq(app_user_id))
        .filter(destination_account.eq(to_account))
        .load::<Transfer>(connection)
        .expect("Error loading transfers")
}

pub fn get_transfer(find_id: i32, app_user_id: i32, connection: &PgConnection) -> Result<Transfer, Error> {
    use crate::database::schema::transfers::dsl::*;

    transfers
        .filter(user_id.eq(app_user_id))
        .filter(id.eq(find_id))
        .first::<Transfer>(connection)
}

pub fn update_transfer(update_id: i32, update_transfer: &NewTransfer, app_user_id: i32, connection: &PgConnection) -> Result<Transfer, Error> {
    use crate::database::schema::transfers::dsl::*;

    diesel::update(transfers.filter(user_id.eq(app_user_id)).find(update_id))
        .set((
            origin_account.eq(update_transfer.origin_account),
            destination_account.eq(update_transfer.destination_account),
            value.eq(update_transfer.value),
            description.eq(update_transfer.description),
            date.eq(update_transfer.date)
        ))
        .get_result::<Transfer>(connection)
}

pub fn delete_transfer(delete_id: i32, app_user_id: i32, connection: &PgConnection) -> Result<Transfer, Error> {
    use crate::database::schema::transfers::dsl::*;

    diesel::delete(transfers.filter(user_id.eq(app_user_id)).find(delete_id))
        .get_result::<Transfer>(connection)
}