use diesel::result::{DatabaseErrorKind, Error};
use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth_guard::Authentication;
use crate::controller_accounts;
use crate::controller_auth;
use crate::db_auth::DatabaseAuth;
use crate::jwt;
use crate::models_db::NewAppUser;
use crate::models_routes::InitialData;

#[post("/login", format = "json", data = "<user>")]
pub fn login(user: Json<NewAppUser>) -> Result<Json<InitialData>, Status> {
    if let Some(initial_data) = controller_auth::login(&user.into_inner()) {
        return Ok(Json(initial_data));
    }

    Err(Status::Unauthorized)
}

#[get("/token")]
pub fn validate_token(auth: Authentication) -> Json<InitialData> {
    Json(InitialData {
        token: jwt::create_jwt(auth.token.claims.user_id),
        accounts: controller_accounts::get_all_accounts(auth.token.claims.user_id),
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

            if let Some(initial_data) = controller_auth::login(&new_app_user) {
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