use crate::{controller_accounts, controller_categories, jwt};
use crate::db_auth::DatabaseAuth;
use crate::models_db::NewAppUser;
use crate::models_routes::InitialData;

pub fn login(user: &NewAppUser) -> Option<InitialData> {
    if let Ok(app_user) = DatabaseAuth::new().authenticate_user(&user) {
        return Some(InitialData {
            token: jwt::create_jwt(app_user.id),
            accounts: controller_accounts::get_all_accounts(app_user.id),
            categories: controller_categories::get_all_categories(app_user.id)
        });
    }

    None
}