use diesel::PgConnection;

use crate::routes::models::GetAccount;
use crate::utils;

pub fn get_all_accounts(user_id: i32, connection: &PgConnection) -> Vec<GetAccount> {
    let accounts = crate::database::accounts::get_all_accounts(user_id, connection);

    let mut accounts_with_balance = Vec::new();

    for account in &accounts {
        let balance = utils::get_account_balance(account.id, user_id, connection);

        accounts_with_balance.push(GetAccount {
            id: account.id,
            name: account.name.clone(),
            balance,
            user_id: user_id,
        });
    }

    accounts_with_balance
}

pub fn get_account(account_id: i32, user_id: i32, connection: &PgConnection) -> Option<GetAccount> {
    if let Ok(account) = crate::database::accounts::get_account(account_id, user_id, connection) {
        return Some(GetAccount {
            id: account.id,
            name: account.name.clone(),
            balance: utils::get_account_balance(account.id, user_id, connection),
            user_id,
        });
    }

    None
}