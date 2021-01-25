use std::env;
use std::str::FromStr;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;

use crate::models::{Account, AppUser, Category, CategoryTypes, NewAccount, NewAppUser, NewCategory, NewTransaction, NewTransfer, Transaction, Transfer};

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

    pub fn new_user(&self, new_user: &NewAppUser) -> QueryResult<AppUser> {
        use crate::schema::app_users;
        use diesel::sql_types::{Integer, Text};

        sql_function!(fn gen_salt(salt_type: Text, iter: Integer) -> Text);
        sql_function!(fn crypt(password: Text, salt: Text) -> Text);

        dotenv().ok();

        let bf_rounds = env::var("BF_ROUNDS")
            .expect("BF_ROUNDS must be set");

        let bf_rounds = i32::from_str(bf_rounds.as_str())
            .expect("BF_ROUNDS must be numeric");

        diesel::insert_into(app_users::table)
            .values((
                app_users::name.eq(new_user.name.clone()),
                app_users::password.eq(crypt(new_user.password.clone(), gen_salt("bf", bf_rounds)))
            ))
            .get_result(&self.connection)
    }

    pub fn new_transfer(&self, new_transfer: &NewTransfer) -> Transfer {
        use crate::schema::transfers;

        diesel::insert_into(transfers::table)
            .values(new_transfer)
            .get_result(&self.connection)
            .expect("Error saving new transfer")
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

    pub fn get_user_by_name(&self, user_name: &str) -> QueryResult<AppUser> {
        use crate::schema::app_users::dsl::*;

        app_users
            .filter(name.eq(user_name))
            .first::<AppUser>(&self.connection)
    }

    pub fn authenticate_user(&self, user: &NewAppUser) -> QueryResult<AppUser> {
        use crate::schema::app_users::dsl::*;
        use diesel::sql_types::Text;

        sql_function!(fn crypt(provided_password: Text, password_in_db: Text) -> Text);

        let user_in_db = self.get_user_by_name(&user.name);

        match user_in_db {
            Ok(user_in_db) => {
                app_users
                    .filter(name.eq(user.name))
                    .filter(password.eq(crypt(user.password, user_in_db.password)))
                    .first::<AppUser>(&self.connection)
            },
            Err(err) => Err(err)
        }
    }

    pub fn get_all_transactions_of_account_joined(&self, account_id: i32, app_user_id: i32) -> Vec<(Transaction, Category, Account)> {
        use crate::schema::transactions::dsl::*;
        use crate::schema::transactions;
        use crate::schema::categories;
        use crate::schema::accounts;

        transactions::table.inner_join(categories::table).inner_join(accounts::table)
            .filter(user_id.eq(app_user_id))
            .filter(account.eq(account_id))
            .order(date.desc())
            .load::<(Transaction, Category, Account)>(&self.connection)
            .expect(format!("Error loading transactions for account {}", account_id).as_str())
    }

    pub fn get_all_accounts(&self, app_user_id: i32) -> Vec<Account> {
        use crate::schema::accounts::dsl::*;

        accounts
            .filter(user_id.eq(app_user_id))
            .load::<Account>(&self.connection)
            .expect("Error loading accounts")
    }

    pub fn get_all_categories(&self, app_user_id: i32) -> Vec<Category> {
        use crate::schema::categories::dsl::*;

        categories
            .filter(user_id.eq(app_user_id))
            .load::<Category>(&self.connection)
            .expect("Error loading categories")
    }

    pub fn get_all_categories_by_type(&self, category_type: CategoryTypes, app_user_id: i32) -> Vec<Category> {
        use crate::schema::categories::dsl::*;

        categories
            .filter(user_id.eq(app_user_id))
            .filter(categorytype.eq(category_type))
            .load::<Category>(&self.connection)
            .expect("Error loading expense categories")
    }

    pub fn get_transfers_from_account(&self, from_account: i32, app_user_id: i32) -> Vec<Transfer> {
        use crate::schema::transfers::dsl::*;

        transfers
            .filter(user_id.eq(app_user_id))
            .filter(origin_account.eq(from_account))
            .load::<Transfer>(&self.connection)
            .expect("Error loading transfers")
    }

    pub fn get_transfers_to_account(&self, to_account: i32, app_user_id: i32) -> Vec<Transfer> {
        use crate::schema::transfers::dsl::*;

        transfers
            .filter(user_id.eq(app_user_id))
            .filter(destination_account.eq(to_account))
            .load::<Transfer>(&self.connection)
            .expect("Error loading transfers")
    }

    pub fn get_transaction(&self, find_id: i32, app_user_id: i32) -> Result<(Transaction, Category, Account), Error> {
        use crate::schema::transactions::dsl::*;
        use crate::schema::transactions;
        use crate::schema::categories;
        use crate::schema::accounts;

        transactions::table.inner_join(categories::table).inner_join(accounts::table)
            .filter(user_id.eq(app_user_id))
            .filter(id.eq(find_id))
            .first::<(Transaction, Category, Account)>(&self.connection)
    }

    pub fn get_account(&self, find_id: i32, app_user_id: i32) -> Result<Account, Error> {
        use crate::schema::accounts::dsl::*;

        accounts
            .filter(user_id.eq(app_user_id))
            .find(find_id)
            .first::<Account>(&self.connection)
    }

    pub fn get_category(&self, find_id: i32, app_user_id: i32) -> Result<Category, Error> {
        use crate::schema::categories::dsl::*;

        categories
            .filter(user_id.eq(app_user_id))
            .find(find_id)
            .first::<Category>(&self.connection)
    }

    pub fn update_transaction(&self, update_id: i32, update_transaction: &NewTransaction, app_user_id: i32) -> Result<Transaction, Error> {
        use crate::schema::transactions::dsl::*;

        diesel::update(transactions.filter(user_id.eq(app_user_id)).find(update_id))
            .set((
                value.eq(update_transaction.value),
                description.eq(update_transaction.description),
                date.eq(update_transaction.date),
                account.eq(update_transaction.account),
                category.eq(update_transaction.category)))
            .get_result::<Transaction>(&self.connection)
    }

    pub fn update_account(&self, update_id: i32, update_account: &NewAccount, app_user_id: i32) -> Result<Account, Error> {
        use crate::schema::accounts::dsl::*;

        diesel::update(accounts.filter(user_id.eq(app_user_id)).find(update_id))
            .set(name.eq(update_account.name))
            .get_result::<Account>(&self.connection)
    }

    pub fn update_category(&self, update_id: i32, update_category: &NewCategory, app_user_id: i32) -> Result<Category, Error> {
        use crate::schema::categories::dsl::*;

        diesel::update(categories.filter(user_id.eq(app_user_id)).find(update_id))
            .set((name.eq(update_category.name), categorytype.eq(update_category.categorytype)))
            .get_result::<Category>(&self.connection)
    }

    pub fn delete_transaction(&self, delete_id: i32, app_user_id: i32) -> Result<Transaction, Error> {
        use crate::schema::transactions::dsl::*;

        diesel::delete(transactions.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<Transaction>(&self.connection)
    }

    pub fn delete_account(&self, delete_id: i32, app_user_id: i32) -> Result<Account, Error> {
        use crate::schema::accounts::dsl::*;

        diesel::delete(accounts.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<Account>(&self.connection)
    }

    pub fn delete_category(&self, delete_id: i32, app_user_id: i32) -> Result<Category, Error> {
        use crate::schema::categories::dsl::*;

        diesel::delete(categories.filter(user_id.eq(app_user_id)).find(delete_id))
            .get_result::<Category>(&self.connection)
    }
}