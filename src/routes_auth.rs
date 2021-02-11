use diesel::result::{DatabaseErrorKind, Error};
use rocket;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::Json;

use crate::auth_guard::Authentication;
use crate::db_auth::DatabaseAuth;
use crate::jwt;
use crate::models_db::NewAppUser;

fn login_internal(user: &NewAppUser) -> Result<String, Status> {
    let result = DatabaseAuth::new().authenticate_user(user);

    match result {
        Ok(user) => {
            Ok(jwt::create_jwt(user.id))
        }
        Err(_) => Err(Status::Unauthorized)
    }
}

#[post("/login", format = "json", data = "<user>")]
pub fn login(user: Json<NewAppUser>) -> Result<String, Status> {
    login_internal(&user.into_inner())
}

#[get("/token")]
pub fn validate_token(_auth: Authentication) -> Status {
    Status::Ok
}

#[post("/users", format = "json", data = "<user_json>")]
pub fn post_user(user_json: Json<NewAppUser>) -> Result<String, Status> {
    let result = DatabaseAuth::new().new_user(&user_json);

    match result {
        Ok(user) => {
            let new_app_user = NewAppUser {
                name: user.name.as_str(),
                password: user_json.password,
            };

            login_internal(&new_app_user)
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