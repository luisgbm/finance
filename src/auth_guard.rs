use jsonwebtoken::TokenData;
use rocket;
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request};

use crate::jwt;
use crate::jwt::Claims;

#[derive(Debug)]
pub struct Authentication {
    pub token: TokenData<Claims>
}

#[derive(Debug)]
pub enum AuthenticationError {
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for Authentication {
    type Error = AuthenticationError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let min_header_len = "Bearer ".len();

        let auth_header = request.headers().get_one("Authorization");

        match auth_header {
            None => Failure((Status::Unauthorized, AuthenticationError::Missing)),
            Some(auth_header) => {
                if auth_header.len() < min_header_len {
                    return Failure((Status::Unauthorized, AuthenticationError::Invalid));
                }

                match jwt::validate_jwt(&auth_header[min_header_len..].to_string()) {
                    Ok(token) => {
                        Success(Authentication {
                            token
                        })
                    }
                    Err(_) => Failure((Status::Unauthorized, AuthenticationError::Invalid))
                }
            }
        }
    }
}