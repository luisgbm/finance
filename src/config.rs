use std::env;
use std::str::FromStr;

/// Application configuration, loaded from environment variables (and a `.env` file if present).
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_validity_days: i64,
    pub bf_rounds: i32,
    pub bind_addr: String,
    pub port: u16,
    pub db_pool_size: u32,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?,
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| anyhow::anyhow!("JWT_SECRET must be set"))?,
            jwt_validity_days: parse_env("JWT_VALIDITY_DAYS", 30)?,
            bf_rounds: parse_env("BF_ROUNDS", 10)?,
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: parse_env("PORT", 8000)?,
            db_pool_size: parse_env("DB_POOL_SIZE", 20)?,
        })
    }
}

fn parse_env<T: FromStr>(key: &str, default: T) -> anyhow::Result<T> {
    match env::var(key) {
        Ok(value) => T::from_str(value.trim())
            .map_err(|_| anyhow::anyhow!("{} must be a valid {}", key, std::any::type_name::<T>())),
        Err(_) => Ok(default),
    }
}
