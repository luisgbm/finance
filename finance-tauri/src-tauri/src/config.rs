use std::env;
use std::str::FromStr;

/// Application configuration for the local desktop build.
///
/// With the move to Tauri IPC there is no JWT any more; callers authenticate with an opaque
/// session token (see `db::sessions`) that the backend resolves to a user id. The only
/// remaining setting is the bcrypt cost used when hashing new passwords. It has a built-in
/// default; the env var remains as an optional override (useful for tests / debugging).
#[derive(Debug, Clone)]
pub struct Config {
    pub bf_rounds: i32,
}

impl Config {
    pub fn local() -> Self {
        Self {
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
