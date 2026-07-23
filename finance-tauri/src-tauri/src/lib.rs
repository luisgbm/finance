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
use tauri_plugin_log::{Target, TargetKind};

/// Show a native error dialog for an unrecoverable startup failure and exit.
///
/// Used before the main window (and the Tauri event loop) exist, so it relies on `rfd`'s
/// synchronous dialog rather than `tauri-plugin-dialog` — the latter dispatches to the main
/// thread via the event loop, which is not yet running inside `setup`, and would deadlock.
fn fatal_startup_error(message: &str) -> ! {
    log::error!("fatal startup error: {message}");
    rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_title("Finance — startup error")
        .set_description(message)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
    std::process::exit(1);
}

/// Application entry point (shared by the desktop binary and any mobile entry point).
///
/// On startup it:
///   1. resolves a writable SQLite path in the OS app-data directory,
///   2. starts the embedded Axum + SQLite backend on an ephemeral loopback port, and
///   3. creates the main window, injecting the backend base URL as a JS global *before*
///      any page script runs so the reused React frontend targets the local API.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    // The single-instance guard must be the first plugin registered. It focuses the
    // already-running window instead of launching a second backend/pool/window.
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }));
    }

    builder
        // Route all `log` records to a rotating file in the OS log dir (and stdout in debug),
        // so diagnostics survive even though the release build hides the console.
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                // Replace (not append to) the default targets, otherwise the default
                // `LogDir { file_name: None }` writer plus ours both target the same
                // case-insensitive file on Windows and every line is written twice.
                .targets([
                    Target::new(TargetKind::LogDir {
                        file_name: Some("finance".into()),
                    }),
                    Target::new(TargetKind::Stdout),
                ])
                .build(),
        )
        .setup(|app| {
            // A per-user, writable location for the database file.
            let data_dir = match app.path().app_data_dir() {
                Ok(dir) => dir,
                Err(err) => {
                    fatal_startup_error(&format!(
                        "Could not resolve the application data directory:\n\n{err}"
                    ))
                }
            };
            std::fs::create_dir_all(&data_dir).ok();
            let db_path = data_dir.join("finance.db");

            // Start the embedded backend and learn which port it bound to. block_on is fine
            // here: setup runs on the main thread, not on an async runtime worker.
            let port = match tauri::async_runtime::block_on(bootstrap::start(&db_path)) {
                Ok(port) => port,
                Err(err) => fatal_startup_error(&format!(
                    "Could not start the local Finance backend:\n\n{err:#}"
                )),
            };

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
