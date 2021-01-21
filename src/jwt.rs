use std::env;
use std::str::FromStr;

use chrono::prelude::*;
use dotenv::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use jsonwebtoken::errors::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    #[serde(with = "jwt_numeric_date")]
    pub iat: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    pub exp: DateTime<Utc>,
}

impl Claims {
    pub fn new(user_id: i32, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        Self {
            user_id,
            iat: iat.date().and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0),
            exp: exp.date().and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0),
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
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single()
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

pub fn create_jwt(user_id: i32) -> String {
    dotenv().ok();

    let jwt_validity_days = env::var("JWT_VALIDITY_DAYS")
        .expect("JWT_VALIDITY_DAYS must be set");

    let jwt_validity_days = i64::from_str(jwt_validity_days.as_str())
        .expect("JWT_VALIDITY_DAYS must be numeric");

    let iat = Utc::now();
    let exp = iat + chrono::Duration::days(jwt_validity_days);

    let claims = Claims::new(user_id, iat, exp);

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    ).unwrap()
}

pub fn validate_jwt(token: &String) -> Result<TokenData<Claims>> {
    dotenv().ok();

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
}