use diesel::PgConnection;

use crate::controllers;
use crate::database::models::NewAppUser;
use crate::routes::models::InitialData;
use crate::utils::jwt;

pub fn login(user: &NewAppUser, connection: &PgConnection) -> Option<InitialData> {
    if let Ok(app_user) = crate::database::auth::authenticate_user(&user, connection) {
        return Some(InitialData {
            token: jwt::create_jwt(app_user.id),
            accounts: controllers::accounts::get_all_accounts(app_user.id, connection),
            categories: controllers::categories::get_all_categories(app_user.id, &*connection),
            scheduled_transactions: controllers::scheduled_transactions::get_all_scheduled_transactions(app_user.id, connection).expect("Error loading scheduled transactions"),
        });
    }

    None
}