mod bootstrap;
mod commands;
mod config;
mod db;
mod error;
mod models;
mod service;
mod state;

#[cfg(test)]
mod tests;

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_log::{Target, TargetKind};

use crate::config::Config;
use crate::state::AppState;

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
/// On startup it opens the SQLite database in the OS app-data directory, registers the
/// shared [`AppState`] so every IPC command can reach the pool, and then creates the main
/// window. The React frontend talks to Rust exclusively through Tauri commands (see
/// `commands.rs`) — there is no local HTTP server or port any more.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    // The single-instance guard must be the first plugin registered. It focuses the
    // already-running window instead of launching a second database/pool/window.
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
                Err(err) => fatal_startup_error(&format!(
                    "Could not resolve the application data directory:\n\n{err}"
                )),
            };
            std::fs::create_dir_all(&data_dir).ok();
            let db_path = data_dir.join("finance.db");

            // Open the pool and apply the schema. block_on is fine here: setup runs on the
            // main thread, not on an async runtime worker.
            let pool = match tauri::async_runtime::block_on(bootstrap::init(&db_path)) {
                Ok(pool) => pool,
                Err(err) => fatal_startup_error(&format!(
                    "Could not open the local Finance database:\n\n{err:#}"
                )),
            };

            // Register shared state *before* the window is created, so the frontend can never
            // load and invoke a command before the pool is available.
            app.manage(AppState {
                pool,
                config: Config::local(),
            });

            WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Finance")
                .inner_size(1280.0, 800.0)
                .build()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::register,
            commands::login,
            commands::get_initial_data,
            commands::create_account,
            commands::get_accounts,
            commands::get_account,
            commands::update_account,
            commands::delete_account,
            commands::create_category,
            commands::get_categories,
            commands::get_categories_by_type,
            commands::get_category,
            commands::update_category,
            commands::delete_category,
            commands::create_transaction,
            commands::get_transactions_for_account,
            commands::get_transaction,
            commands::update_transaction,
            commands::delete_transaction,
            commands::create_transfer,
            commands::get_transfer,
            commands::update_transfer,
            commands::delete_transfer,
            commands::create_scheduled_transaction,
            commands::get_scheduled_transactions,
            commands::get_scheduled_transaction,
            commands::update_scheduled_transaction,
            commands::delete_scheduled_transaction,
            commands::pay_scheduled_transaction,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the Finance application");
}
