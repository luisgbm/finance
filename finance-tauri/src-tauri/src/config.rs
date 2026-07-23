use std::env;
use std::str::FromStr;

/// Application configuration for the local desktop build.
///
/// The original server loaded these from environment variables / a `.env` file. In the
/// desktop POC there is a single local user-facing process, so everything has a built-in
/// default; the env vars remain as optional overrides (useful for tests / debugging).
///
/// `jwt_secret` is intentionally a fixed local value so the token persisted by the web
/// frontend (in localStorage) keeps working across app restarts. `jwt_validity_days` is
/// large for the same reason.
#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_validity_days: i64,
    pub bf_rounds: i32,
}

impl Config {
    pub fn local() -> Self {
        Self {
            jwt_secret: env::var("FINANCE_JWT_SECRET")
                .unwrap_or_else(|_| "finance-tauri-local-poc-secret".to_string()),
            jwt_validity_days: parse_env("FINANCE_JWT_VALIDITY_DAYS", 3650),
            bf_rounds: parse_env("FINANCE_BF_ROUNDS", 10),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::local()
    }
}

fn parse_env<T: FromStr>(key: &str, default: T) -> T {
    match env::var(key) {
        Ok(value) => T::from_str(value.trim()).unwrap_or(default),
        Err(_) => default,
    }
}
