use diesel::prelude::*;
use diesel::result::Error;

use crate::database::models::{Account, NewAccount};

pub fn new_account(new_account: &NewAccount, connection: &PgConnection) -> Account {
    use crate::database::schema::accounts;

    diesel::insert_into(accounts::table)
        .values(new_account)
        .get_result(connection)
        .expect("Error saving new account")
}

pub fn get_all_accounts(app_user_id: i32, connection: &PgConnection) -> Vec<Account> {
    use crate::database::schema::accounts::dsl::*;

    accounts
        .filter(user_id.eq(app_user_id))
        .load::<Account>(connection)
        .expect("Error loading accounts")
}

pub fn get_account(find_id: i32, app_user_id: i32, connection: &PgConnection) -> Result<Account, Error> {
    use crate::database::schema::accounts::dsl::*;

    accounts
        .filter(user_id.eq(app_user_id))
        .find(find_id)
        .first::<Account>(connection)
}

pub fn update_account(update_id: i32, update_account: &NewAccount, app_user_id: i32, connection: &PgConnection) -> Result<Account, Error> {
    use crate::database::schema::accounts::dsl::*;

    diesel::update(accounts.filter(user_id.eq(app_user_id)).find(update_id))
        .set(name.eq(update_account.name))
        .get_result::<Account>(connection)
}

pub fn delete_account(delete_id: i32, app_user_id: i32, connection: &PgConnection) -> Result<Account, Error> {
    use crate::database::schema::accounts::dsl::*;

    diesel::delete(accounts.filter(user_id.eq(app_user_id)).find(delete_id))
        .get_result::<Account>(connection)
}