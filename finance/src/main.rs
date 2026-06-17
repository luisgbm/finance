mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod service;
mod state;

use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env if present (ignored if missing), then initialise structured logging.
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env()?;

    let pool = PgPoolOptions::new()
        .max_connections(config.db_pool_size)
        .connect(&config.database_url)
        .await
        .context("failed to connect to the database")?;

    let bind = format!("{}:{}", config.bind_addr, config.port);
    let state = AppState {
        pool,
        config: Arc::new(config),
    };

    let app = Router::new()
        .merge(handlers::auth::routes())
        .merge(handlers::categories::routes())
        .merge(handlers::accounts::routes())
        .merge(handlers::transactions::routes())
        .merge(handlers::transfers::routes())
        .merge(handlers::scheduled_transactions::routes())
        // Permissive CORS mirrors the original `rocket_cors` default (any origin/method/header);
        // the API authenticates via a Bearer token, not cookies, so credentials are not needed.
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .with_context(|| format!("failed to bind {bind}"))?;

    tracing::info!("finance backend listening on http://{bind}");

    axum::serve(listener, app)
        .await
        .context("server error")?;

    Ok(())
}
