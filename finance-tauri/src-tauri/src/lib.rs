mod bootstrap;
mod commands;
mod db;
mod error;
mod models;
mod service;
mod state;

#[cfg(test)]
mod tests;

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_log::{Target, TargetKind};

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

/// Install a global panic hook that routes panics into the log file before the process dies.
///
/// The release profile builds with `panic = "abort"`, so a panic terminates the app almost
/// immediately and the WebView shows nothing. Without this hook the crash would leave no
/// trace; here we record the panic message, thread and source location to the same rotating
/// log file as everything else, then delegate to the default hook (which prints to stderr in
/// debug builds). This is diagnostics only — it does not attempt to recover.
fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());

        let message = match info.payload().downcast_ref::<&str>() {
            Some(s) => (*s).to_string(),
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => s.clone(),
                None => "Box<dyn Any>".to_string(),
            },
        };

        let thread = std::thread::current();
        let thread_name = thread.name().unwrap_or("unnamed");

        log::error!("panic in thread '{thread_name}' at {location}: {message}");

        default_hook(info);
    }));
}

/// Build the tauri-specta command registry shared by the runtime invoke handler (in [`run`])
/// and the binding-export test, so the generated TypeScript can never cover a different set of
/// commands than the app actually serves. Adding a command here is the single place it needs
/// to be registered for both wiring and type generation.
pub(crate) fn specta_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new().commands(tauri_specta::collect_commands![
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
}

/// Application entry point (shared by the desktop binary and any mobile entry point).
///
/// On startup it opens the SQLite database in the OS app-data directory, registers the
/// shared [`AppState`] so every IPC command can reach the pool, and then creates the main
/// window. The React frontend talks to Rust exclusively through Tauri commands (see
/// `commands.rs`) — there is no local HTTP server or port any more.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Route panics into the log file (see the hook's doc comment). Installed first so it
    // covers failures during startup too.
    install_panic_hook();

    // Type-safe IPC: tauri-specta introspects the command signatures to build the invoke handler
    // wired into Tauri. In debug builds it additionally emits a TypeScript `bindings.ts` the
    // frontend imports for compile-time-checked command names, arguments and return types.
    let specta_builder = specta_builder();

    // Keep the generated bindings in sync with the Rust commands — debug-only, so the release
    // binary carries neither the exporter nor this dev tooling.
    #[cfg(debug_assertions)]
    {
        // Headless regeneration hook: `cargo run -- --export-bindings` (or the debug exe with the
        // same flag) writes `bindings.ts` via pure `specta` type introspection and exits before any
        // window or database is created — the deterministic, GUI-free way to refresh the bindings.
        // (The app binary loads cleanly here, whereas pulling the tauri-specta builder into a
        // unit-test binary trips a WebView2 entry-point mismatch on Windows, so a test can't.)
        if std::env::args().any(|arg| arg == "--export-bindings") {
            match specta_builder.export(
                specta_typescript::Typescript::default(),
                "../src/api/bindings.ts",
            ) {
                Ok(()) => {
                    println!("wrote TypeScript bindings to ../src/api/bindings.ts");
                    std::process::exit(0);
                }
                Err(err) => {
                    eprintln!("failed to export TypeScript bindings: {err}");
                    std::process::exit(1);
                }
            }
        }

        // Also regenerate on every `tauri dev` startup (CWD = `src-tauri`, so the relative path
        // resolves) so the bindings can't drift while developing. Non-fatal: a standalone `--debug`
        // build launched from another directory can't resolve the path, and a panic here would stop
        // the app from starting.
        if let Err(err) = specta_builder.export(
            specta_typescript::Typescript::default(),
            "../src/api/bindings.ts",
        ) {
            log::warn!("could not export TypeScript bindings (expected outside `tauri dev`): {err}");
        }
    }

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
            app.manage(AppState { pool });

            WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Finance")
                .inner_size(1280.0, 800.0)
                .build()?;

            Ok(())
        })
        .invoke_handler(specta_builder.invoke_handler())
        .build(tauri::generate_context!())
        .expect("error while building the Finance application")
        .run(|app_handle, event| {
            // On shutdown, flush the write-ahead log back into the main database file so the
            // single finance.db is self-contained (no leftover -wal/-shm side files still
            // holding committed rows). Best-effort — a failure here must not block exit.
            if let tauri::RunEvent::Exit = event {
                checkpoint_wal(app_handle);
            }
        });
}

/// Flush and truncate the SQLite write-ahead log so every committed row lives in the main
/// `finance.db` file after the app closes. Invoked from the `RunEvent::Exit` handler on the
/// main thread, where briefly blocking on this small checkpoint query is fine.
fn checkpoint_wal(app_handle: &tauri::AppHandle) {
    let pool = app_handle.state::<AppState>().pool.clone();
    let result = tauri::async_runtime::block_on(async move {
        sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)")
            .execute(&pool)
            .await
    });
    match result {
        Ok(_) => log::info!("flushed the write-ahead log into the database file on exit"),
        Err(err) => log::warn!("WAL checkpoint on exit failed: {err}"),
    }
}
