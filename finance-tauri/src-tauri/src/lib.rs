mod auth;
pub mod bootstrap;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod service;
mod state;

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tracing_subscriber::EnvFilter;

/// Application entry point (shared by the desktop binary and any mobile entry point).
///
/// On startup it:
///   1. resolves a writable SQLite path in the OS app-data directory,
///   2. starts the embedded Axum + SQLite backend on an ephemeral loopback port, and
///   3. creates the main window, injecting the backend base URL as a JS global *before*
///      any page script runs so the reused React frontend targets the local API.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .try_init()
        .ok();

    tauri::Builder::default()
        .setup(|app| {
            // A per-user, writable location for the database file.
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve the app data directory");
            std::fs::create_dir_all(&data_dir).ok();
            let db_path = data_dir.join("finance.db");

            // Start the embedded backend and learn which port it bound to. block_on is fine
            // here: setup runs on the main thread, not on an async runtime worker.
            let port = tauri::async_runtime::block_on(bootstrap::start(&db_path))
                .expect("failed to start the embedded backend");

            // Runs before the page's own scripts, so window.__FINANCE_API_BASE__ is already
            // set by the time src/api/finance.js constructs its axios client.
            let init_script =
                format!("window.__FINANCE_API_BASE__ = 'http://127.0.0.1:{port}/api';");

            WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Finance")
                .inner_size(1280.0, 800.0)
                .initialization_script(&init_script)
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the Finance application");
}
