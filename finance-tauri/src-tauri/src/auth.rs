use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use chrono::prelude::*;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::state::AppState;

/// JWT claims. The shape (and the numeric-date serialization of `iat`/`exp`) is kept
/// byte-for-byte compatible with the original Rocket backend so previously issued
/// tokens remain valid under the same `JWT_SECRET`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    #[serde(with = "jwt_numeric_date")]
    pub iat: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    pub exp: DateTime<Utc>,
}

impl Claims {
    fn new(user_id: i32, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        // Truncate to whole seconds (drop sub-second precision), matching the original.
        Self {
            user_id,
            iat: iat.with_nanosecond(0).unwrap_or(iat),
            exp: exp.with_nanosecond(0).unwrap_or(exp),
        }
    }
}

mod jwt_numeric_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single()
            .ok_or_else(|| serde::de::Error::custom("Invalid UNIX timestamp value"))
    }
}

pub fn create_jwt(user_id: i32, secret: &str, validity_days: i64) -> Result<String, AppError> {
    let iat = Utc::now();
    let exp = iat + chrono::Duration::days(validity_days);
    let claims = Claims::new(user_id, iat, exp);

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::Internal(format!("failed to encode JWT: {e}")))
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, AppError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_aud = false;

    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map_err(|_| AppError::Unauthorized)
}

/// Authenticated user, extracted from the `Authorization: Bearer <token>` header.
/// Mirrors the original Rocket `Authentication` request guard.
pub struct AuthUser {
    pub user_id: i32,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        // Strip the "Bearer " prefix; reject anything too short to contain a token.
        let token = header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        let token_data = validate_jwt(token, &state.config.jwt_secret)?;

        Ok(AuthUser {
            user_id: token_data.claims.user_id,
        })
    }
}
