use crate::db_accounts::DatabaseAccounts;
use crate::models_routes::GetAccount;
use crate::utils;

pub fn get_all_accounts(user_id: i32) -> Vec<GetAccount> {
    let accounts = DatabaseAccounts::new().get_all_accounts(user_id);

    let mut accounts_with_balance = Vec::new();

    for account in &accounts {
        let balance = utils::get_account_balance(account.id, user_id);

        accounts_with_balance.push(GetAccount {
            id: account.id,
            name: account.name.clone(),
            balance,
            user_id: user_id,
        });
    }

    accounts_with_balance
}

pub fn get_account(account_id: i32, user_id: i32) -> Option<GetAccount> {
    if let Ok(account) = DatabaseAccounts::new().get_account(account_id, user_id) {
        return Some(GetAccount {
            id: account.id,
            name: account.name.clone(),
            balance: utils::get_account_balance(account.id, user_id),
            user_id,
        });
    }

    None
}