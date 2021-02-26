use diesel::result::{DatabaseErrorKind, Error};
use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::controllers;
use crate::database::auth::DatabaseAuth;
use crate::database::models::NewAppUser;
use crate::routes::auth_guard::Authentication;
use crate::routes::models::InitialData;
use crate::utils::jwt;

#[post("/login", format = "json", data = "<user>")]
pub fn login(user: Json<NewAppUser>) -> Result<Json<InitialData>, Status> {
    if let Some(initial_data) = controllers::auth::login(&user.into_inner()) {
        return Ok(Json(initial_data));
    }

    Err(Status::Unauthorized)
}

#[get("/token")]
pub fn validate_token(auth: Authentication) -> Json<InitialData> {
    let user_id = auth.token.claims.user_id;

    Json(InitialData {
        token: jwt::create_jwt(user_id),
        accounts: controllers::accounts::get_all_accounts(user_id),
        categories: controllers::categories::get_all_categories(user_id),
        scheduled_transactions: controllers::scheduled_transactions::get_all_scheduled_transactions(user_id).expect("Error loading scheduled transactions"),
    })
}

#[post("/users", format = "json", data = "<user_json>")]
pub fn post_user(user_json: Json<NewAppUser>) -> Result<Json<InitialData>, Status> {
    let result = DatabaseAuth::new().new_user(&user_json);

    match result {
        Ok(user) => {
            let new_app_user = NewAppUser {
                name: user.name.as_str(),
                password: user_json.password,
            };

            if let Some(initial_data) = controllers::auth::login(&new_app_user) {
                Ok(Json(initial_data))
            } else {
                Err(Status::Unauthorized)
            }
        }
        Err(err) => {
            match err {
                Error::DatabaseError(error_kind, _) => {
                    match error_kind {
                        DatabaseErrorKind::UniqueViolation => Err(Status::Conflict),
                        _ => Err(Status::InternalServerError)
                    }
                }
                Error::NotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError)
            }
        }
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![
        login,
        validate_token,
        post_user
    ]
}