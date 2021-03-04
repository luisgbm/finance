use diesel::prelude::*;
use diesel::result::Error;

use crate::database::models::{Account, Category, NewTransaction, Transaction};

pub fn new_transaction(new_transaction: &NewTransaction, connection: &PgConnection) -> Transaction {
    use crate::database::schema::transactions;

    diesel::insert_into(transactions::table)
        .values(new_transaction)
        .get_result(connection)
        .expect("Error saving new transaction")
}

pub fn get_all_transactions_of_account_joined(account_id: i32, app_user_id: i32, connection: &PgConnection) -> Vec<(Transaction, Category, Account)> {
    use crate::database::schema::transactions::dsl::*;
    use crate::database::schema::transactions;
    use crate::database::schema::categories;
    use crate::database::schema::accounts;

    transactions::table.inner_join(categories::table).inner_join(accounts::table)
        .filter(user_id.eq(app_user_id))
        .filter(account.eq(account_id))
        .order(date.desc())
        .load::<(Transaction, Category, Account)>(connection)
        .expect(format!("Error loading transactions for account {}", account_id).as_str())
}

pub fn get_transaction(find_id: i32, app_user_id: i32, connection: &PgConnection) -> Result<(Transaction, Category, Account), Error> {
    use crate::database::schema::transactions::dsl::*;
    use crate::database::schema::transactions;
    use crate::database::schema::categories;
    use crate::database::schema::accounts;

    transactions::table.inner_join(categories::table).inner_join(accounts::table)
        .filter(user_id.eq(app_user_id))
        .filter(id.eq(find_id))
        .first::<(Transaction, Category, Account)>(connection)
}

pub fn update_transaction(update_id: i32, update_transaction: &NewTransaction, app_user_id: i32, connection: &PgConnection) -> Result<Transaction, Error> {
    use crate::database::schema::transactions::dsl::*;

    diesel::update(transactions.filter(user_id.eq(app_user_id)).find(update_id))
        .set((
            value.eq(update_transaction.value),
            description.eq(update_transaction.description),
            date.eq(update_transaction.date),
            account.eq(update_transaction.account),
            category.eq(update_transaction.category)
        ))
        .get_result::<Transaction>(connection)
}

pub fn delete_transaction(delete_id: i32, app_user_id: i32, connection: &PgConnection) -> Result<Transaction, Error> {
    use crate::database::schema::transactions::dsl::*;

    diesel::delete(transactions.filter(user_id.eq(app_user_id)).find(delete_id))
        .get_result::<Transaction>(connection)
}