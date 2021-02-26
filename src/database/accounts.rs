use diesel::prelude::*;
use diesel::result::Error;

use crate::database::finance::FinanceDB;
use crate::database::models::{Account, NewAccount};

pub struct DatabaseAccounts {
    connection: FinanceDB
}

impl DatabaseAccounts {
    pub fn new() -> DatabaseAccounts {
        DatabaseAccounts {
            connection: FinanceDB::new()
        }
    }

    pub fn new_account(&self, new_account: &NewAccount) -> Account {
        use crate::database::schema::accounts;

        diesel::insert_into(accounts::table)
            .values(new_account)
            .get_result(&self.connection.db_connection)
            .expect("Error saving new account")
    }

    pub fn get_all_accounts(&self, app_user_id: i32) -> Vec<Account> {
        use crate::database::schema::accounts::dsl::*;

        accounts
            .filter(user_id.eq(app_user_id))
            .load::<Account>(&self.connection.db_connection)
            .expect("Error loading accounts")
    }

    pub fn get_account(&self, find_id: i32, app_user_id: i32) -> Result<Account, Error> {
        use crate::database::schema::accounts::dsl::*;

        accounts
            .filter(user_id.eq(app_user_id))
            .find(find_id)
            .first::<Account>(&self.connection.db_connection)
    }

    pub fn update_account(&self, update_id: i32, update_account: &NewAccount, app_user_id: i32) -> Result<Account, Error> {
        use crate::database::schema::accounts::dsl::*;

        diesel::update(accounts.filter(user_id.eq(app_user_id)).find(update_id))
            .set(name.eq(update_account.name))
            .get_result::<Account>(&self.connection.db_connection)
    }

    pub fn delete_account(&self, delete_id: i32, app_user_id: i32) -> Result<Account, Error> {
        use crate::database::schema::accounts::dsl::*;

        diesel::delete(accounts.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<Account>(&self.connection.db_connection)
    }
}