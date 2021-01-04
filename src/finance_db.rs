use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::result::Error;

use crate::models::{NewCategory, Category, Account, NewAccount, NewTransaction, Transaction};

pub struct FinanceDB {
    connection: PgConnection
}

impl FinanceDB {
    pub fn new() -> FinanceDB {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        FinanceDB {
            connection
        }
    }

    pub fn new_transaction(&self, new_transaction: &NewTransaction) -> Transaction {
        use crate::schema::transactions;

        diesel::insert_into(transactions::table)
            .values(new_transaction)
            .get_result(&self.connection)
            .expect("Error saving new transaction")
    }

    pub fn new_account(&self, new_account: &NewAccount) -> Account {
        use crate::schema::accounts;

        diesel::insert_into(accounts::table)
            .values(new_account)
            .get_result(&self.connection)
            .expect("Error saving new account")
    }

    pub fn new_category(&self, new_category: &NewCategory) -> Category {
        use crate::schema::categories;

        diesel::insert_into(categories::table)
            .values(new_category)
            .get_result(&self.connection)
            .expect("Error saving new category")
    }

    pub fn get_all_transactions_of_account(&self, account_id: i32) -> Vec<Transaction> {
        use crate::schema::transactions::dsl::*;

        transactions
            .filter(account.eq(account_id))
            .load::<Transaction>(&self.connection)
            .expect(format!("Error loading transactions for account {}", account_id).as_str())
    }

    pub fn get_all_accounts(&self) -> Vec<Account> {
        use crate::schema::accounts::dsl::*;

        accounts
            .load::<Account>(&self.connection)
            .expect("Error loading accounts")
    }

    pub fn get_all_categories(&self) -> Vec<Category> {
        use crate::schema::categories::dsl::*;

        categories
            .load::<Category>(&self.connection)
            .expect("Error loading categories")
    }

    pub fn get_transaction(&self, find_id: i32) -> Result<Transaction, Error> {
        use crate::schema::transactions::dsl::*;

        transactions
            .find(find_id)
            .first::<Transaction>(&self.connection)
    }

    pub fn get_account(&self, find_id: i32) -> Result<Account, Error> {
        use crate::schema::accounts::dsl::*;

        accounts
            .find(find_id)
            .first::<Account>(&self.connection)
    }

    pub fn get_category(&self, find_id: i32) -> Result<Category, Error> {
        use crate::schema::categories::dsl::*;

        categories
            .find(find_id)
            .first::<Category>(&self.connection)
    }

    pub fn update_transaction(&self, update_id: i32, update_transaction: &NewTransaction) -> Result<Transaction, Error> {
        use crate::schema::transactions::dsl::*;

        diesel::update(transactions.find(update_id))
            .set((
                 value.eq(update_transaction.value),
           description.eq(update_transaction.description),
                  date.eq(update_transaction.date),
               account.eq(update_transaction.account),
              category.eq(update_transaction.category)))
            .get_result::<Transaction>(&self.connection)
    }

    pub fn update_account(&self, update_id: i32, update_account: &NewAccount) -> Result<Account, Error> {
        use crate::schema::accounts::dsl::*;

        diesel::update(accounts.find(update_id))
            .set(name.eq(update_account.name))
            .get_result::<Account>(&self.connection)
    }

    pub fn update_category(&self, update_id: i32, update_category: &NewCategory) -> Result<Category, Error> {
        use crate::schema::categories::dsl::*;

        diesel::update(categories.find(update_id))
            .set((name.eq(update_category.name), categorytype.eq(update_category.categorytype)))
            .get_result::<Category>(&self.connection)
    }

    pub fn delete_transaction(&self, delete_id: i32) -> Result<Transaction, Error> {
        use crate::schema::transactions::dsl::*;

        diesel::delete(transactions.find(delete_id))
            .get_result::<Transaction>(&self.connection)
    }

    pub fn delete_account(&self, delete_id: i32) -> Result<Account, Error> {
        use crate::schema::accounts::dsl::*;

        diesel::delete(accounts.find(delete_id))
            .get_result::<Account>(&self.connection)
    }

    pub fn delete_category(&self, delete_id: i32) -> Result<Category, Error> {
        use crate::schema::categories::dsl::*;

        diesel::delete(categories.find(delete_id))
            .get_result::<Category>(&self.connection)
    }
}