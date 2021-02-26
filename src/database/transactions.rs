use diesel::prelude::*;
use diesel::result::Error;

use crate::database::finance::FinanceDB;
use crate::database::models::{Account, Category, NewTransaction, Transaction};

pub struct DatabaseTransactions {
    connection: FinanceDB
}

impl DatabaseTransactions {
    pub fn new() -> DatabaseTransactions {
        DatabaseTransactions {
            connection: FinanceDB::new()
        }
    }

    pub fn new_transaction(&self, new_transaction: &NewTransaction) -> Transaction {
        use crate::database::schema::transactions;

        diesel::insert_into(transactions::table)
            .values(new_transaction)
            .get_result(&self.connection.db_connection)
            .expect("Error saving new transaction")
    }

    pub fn get_all_transactions_of_account_joined(&self, account_id: i32, app_user_id: i32) -> Vec<(Transaction, Category, Account)> {
        use crate::database::schema::transactions::dsl::*;
        use crate::database::schema::transactions;
        use crate::database::schema::categories;
        use crate::database::schema::accounts;

        transactions::table.inner_join(categories::table).inner_join(accounts::table)
            .filter(user_id.eq(app_user_id))
            .filter(account.eq(account_id))
            .order(date.desc())
            .load::<(Transaction, Category, Account)>(&self.connection.db_connection)
            .expect(format!("Error loading transactions for account {}", account_id).as_str())
    }

    pub fn get_transaction(&self, find_id: i32, app_user_id: i32) -> Result<(Transaction, Category, Account), Error> {
        use crate::database::schema::transactions::dsl::*;
        use crate::database::schema::transactions;
        use crate::database::schema::categories;
        use crate::database::schema::accounts;

        transactions::table.inner_join(categories::table).inner_join(accounts::table)
            .filter(user_id.eq(app_user_id))
            .filter(id.eq(find_id))
            .first::<(Transaction, Category, Account)>(&self.connection.db_connection)
    }

    pub fn update_transaction(&self, update_id: i32, update_transaction: &NewTransaction, app_user_id: i32) -> Result<Transaction, Error> {
        use crate::database::schema::transactions::dsl::*;

        diesel::update(transactions.filter(user_id.eq(app_user_id)).find(update_id))
            .set((
                value.eq(update_transaction.value),
                description.eq(update_transaction.description),
                date.eq(update_transaction.date),
                account.eq(update_transaction.account),
                category.eq(update_transaction.category)
            ))
            .get_result::<Transaction>(&self.connection.db_connection)
    }

    pub fn delete_transaction(&self, delete_id: i32, app_user_id: i32) -> Result<Transaction, Error> {
        use crate::database::schema::transactions::dsl::*;

        diesel::delete(transactions.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<Transaction>(&self.connection.db_connection)
    }
}