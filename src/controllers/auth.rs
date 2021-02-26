use crate::controllers;
use crate::database::auth::DatabaseAuth;
use crate::database::models::NewAppUser;
use crate::routes::models::InitialData;
use crate::utils::jwt;

pub fn login(user: &NewAppUser) -> Option<InitialData> {
    if let Ok(app_user) = DatabaseAuth::new().authenticate_user(&user) {
        return Some(InitialData {
            token: jwt::create_jwt(app_user.id),
            accounts: controllers::accounts::get_all_accounts(app_user.id),
            categories: controllers::categories::get_all_categories(app_user.id),
            scheduled_transactions: controllers::scheduled_transactions::get_all_scheduled_transactions(app_user.id).expect("Error loading scheduled transactions"),
        });
    }

    None
}