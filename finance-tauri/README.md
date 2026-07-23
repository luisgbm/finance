# Finance — Desktop app (Tauri POC)

A **proof‑of‑concept** that repackages Finance as a **completely local desktop app**, distributed as
a **single `.exe`** with no server, no Docker, and no external database. It wraps the existing React
frontend in a [**Tauri 2**](https://tauri.app) shell, ports the original Axum REST API's business
logic to **native Tauri IPC commands**, and swaps **PostgreSQL for embedded SQLite**, so the whole
personal money manager runs offline from one file.

> This is an experiment that lives alongside the production monorepo. The
> [`finance/`](../finance) (Axum + PostgreSQL API) and [`financejs/`](../financejs) (React web app)
> projects remain the canonical apps; this directory reuses their code almost verbatim. See the
> [monorepo README](../README.md) for the Docker‑based web stack.

---

## How it works

Everything runs **in one process**. There is no backend server, no HTTP, and no port — the Rust side
exposes the domain logic as **Tauri commands** the WebView calls through the built‑in `invoke` IPC
bridge, and talks to SQLite in‑process:

1. On launch, the Rust `setup()` resolves a writable SQLite path in the OS **app‑data directory**
   (`%APPDATA%\com.luisgbm.finance\finance.db` on Windows) and creates the schema if missing.
2. It opens the `SqlitePool` and **registers it as shared Tauri state before the window is built**, so
   the frontend can never load and `invoke` a command before the pool exists.
3. It creates the **WebView2** window. The reused React app calls `invoke('command', args)` instead of
   issuing axios HTTP requests; each call is routed to a `#[tauri::command]` handler
   (see [`commands.rs`](./src-tauri/src/commands.rs)) that runs a query and returns JSON‑serializable
   data — or a serialized error the frontend reshapes to look like an axios error.
4. The React SPA behaves exactly like the web app, only every former `/api` call is now a local IPC
   command with **zero network involved**.

```
                         finance-tauri.exe  (one process)
   +----------------------------------------------------------------------+
   |                                                                      |
   |   WebView2 window                     Rust core (Tauri commands)     |
   |   +------------------------+          +------------------------+     |
   |   | React SPA (MUI/Vite)   |  invoke  | #[tauri::command]      |     |
   |   |                        | -------> | (commands -> service   |     |
   |   | @tauri-apps/api core   |   IPC    |  -> db)                |     |
   |   |   invoke('cmd', args)  |   JSON   |                        |     |
   |   |                        | <------- |     |  SQLx (bundled)  |     |
   |   +------------------------+          +-----|------------------+     |
   |                                             v                        |
   |                                       finance.db  (SQLite file,      |
   |                                       in %APPDATA%\com.luisgbm.finance)
   +----------------------------------------------------------------------+

   No network server, no HTTP, no port, no Docker, no PostgreSQL. The frontend
   reaches Rust only through Tauri IPC; the database is a single local SQLite file.
```

The domain (accounts, categories, transactions, transfers, scheduled transactions, computed
balances, integer‑cents money) is unchanged — see [`finance/README.md`](../finance/README.md) for the
full model.

---

## What changed vs. the web app

The port was deliberately minimal. The **frontend** is the `financejs` React app reused almost
verbatim; the components are untouched. The only functional changes are in the API layer under
[`src/api/`](./src/api): [`finance.js`](./src/api/finance.js) is now a thin adapter over Tauri's
`invoke` that shapes results to look like the axios responses the app was written against (`{ data }`
on success; a thrown `Error` with `err.response.status` on failure, so every existing
`err.response.status` check keeps working), and each resource service calls `invoke('command', …)`
instead of an axios HTTP method. A Tauri‑tuned [`vite.config.js`](./vite.config.js) (`base: './'`,
fixed port `1420`, `REACT_APP_` env prefix) rounds it out.

The **backend logic** is the original Axum API, ported to SQLite and re‑exposed as Tauri commands:

| Original (Postgres + Axum) | Desktop POC (SQLite + Tauri IPC) |
|---|---|
| `PgPool` | `SqlitePool` (bundled SQLite, compiled from source — no external DLL) |
| `$1, $2 …` placeholders | `?` placeholders |
| `pgcrypto` (`crypt`/`gen_salt`) password hashing | [`bcrypt`](https://crates.io/crates/bcrypt) crate, in‑process |
| `CREATE TYPE … ENUM` types | plain `TEXT` columns (snake_case enum names) |
| shared `transactions_transfers_id_seq` sequence | a `seq_tx_tr` table drawn from inside a transaction |
| `ON DELETE CASCADE` | same, with `PRAGMA foreign_keys = ON` per connection |
| balance via one aggregate SQL query | three `query_scalar::<i64>` sums combined in Rust |
| Axum routes + handlers over HTTP | `#[tauri::command]` functions invoked over IPC (no router, no port) |
| JWT bearer auth (`jsonwebtoken`) + CORS | no tokens: the frontend passes the logged‑in `user_id` as a command argument. `InitialData.token` is repurposed to carry that id so a persisted login survives restarts |
| env‑driven config (`DATABASE_URL`, `JWT_SECRET` …) | built‑in local defaults in [`config.rs`](./src-tauri/src/config.rs) (only the bcrypt cost remains) |

The SQLite schema lives in [`src-tauri/src/schema.sql`](./src-tauri/src/schema.sql) and is embedded
into the binary (`include_str!`) and applied idempotently on every launch.

### Desktop hardening

Beyond the transport swap, the POC adds a few desktop‑app essentials:

- **Startup‑error dialog** — unrecoverable failures before the window exists (e.g. the database can't
  be opened) show a native `rfd` message box and exit instead of failing silently.
- **File logging** — [`tauri-plugin-log`](https://github.com/tauri-apps/plugins-workspace) writes to
  `%LOCALAPPDATA%\com.luisgbm.finance\logs\finance.log` (and stdout in debug); the frontend forwards
  its `console` output into the same log via [`src/logging.js`](./src/logging.js).
- **Single‑instance guard** — a second launch focuses the running window instead of opening a second
  database/window.
- **Content‑Security‑Policy** — a restrictive CSP in [`tauri.conf.json`](./src-tauri/tauri.conf.json)
  limits `connect-src` to Tauri's own IPC (`'self' ipc: http://ipc.localhost`).

---

## Tech stack

- **Shell:** [Tauri 2](https://tauri.app) (WebView2 on Windows) — window, packaging, app‑data paths, IPC
- **Frontend:** React 19 · MUI 9 · Vite 8 (the `financejs` app, reused) · `@tauri-apps/api` (`invoke`)
- **Backend (embedded):** native `#[tauri::command]` functions on Tauri's async runtime (Tokio) — no HTTP server
- **Database:** [SQLx](https://github.com/launchbadge/sqlx) 0.8 with the **bundled `sqlite`** driver + local `finance.db`
- **Auth:** in‑process [`bcrypt`](https://crates.io/crates/bcrypt) 0.15 password hashing (no JWT)
- **Desktop plugins:** `tauri-plugin-log` (file logging), `tauri-plugin-single-instance`, `rfd` (native dialogs)
- **Other:** `serde`/`serde_json`, `chrono`/`chronoutil`, `log`, `thiserror`, `anyhow`

---

## Project layout

```
finance-tauri/
├── index.html            # Vite entry for the React app
├── vite.config.js        # Tauri-tuned Vite config (base './', port 1420)
├── package.json          # frontend deps + the `tauri` script
├── public/               # static assets (favicon, logos, manifest)
├── src/                  # the reused React SPA (only src/api/ + logging.js changed)
│   ├── api/              # Tauri IPC services; finance.js is the invoke() adapter
│   ├── logging.js        # forwards the WebView console to tauri-plugin-log
│   ├── components/       # accounts, categories, transactions, transfers, scheduled, users
│   ├── redux/  context/  utils/
│   └── main.jsx
└── src-tauri/            # the Rust/Tauri side
    ├── Cargo.toml        # crate deps + release profile (LTO, strip, panic=abort)
    ├── tauri.conf.json   # identifier, devUrl/frontendDist, CSP, bundle config
    ├── build.rs          # tauri-build
    ├── capabilities/     # Tauri capability/permission set (core + log)
    ├── icons/            # generated app icons (from app-icon.png)
    └── src/
        ├── main.rs       # binary entry (hides console in release) -> lib::run()
        ├── lib.rs        # Tauri setup(): open SQLite, manage state, register commands, build window
        ├── commands.rs   # the #[tauri::command] IPC surface, one fn per operation
        ├── bootstrap.rs  # init(): open SQLite pool + apply schema
        ├── schema.sql    # embedded SQLite DDL
        ├── config.rs state.rs error.rs models.rs service.rs
        ├── db/           # SQLx query modules (users, accounts, categories, …)
        └── tests.rs      # in-crate integration test over the db/service/command layer
```

---

## Prerequisites (build)

Unlike the Dockerized web stack, building the desktop app needs a **local toolchain** (Windows):

- **Node.js** (18+) and npm — builds the frontend.
- **Rust** (stable) with `cargo` — builds the Tauri app.
- **Microsoft C++ Build Tools (MSVC)** — the C toolchain used to compile the bundled SQLite.
- **WebView2 Runtime** — preinstalled on current Windows 10/11 (the app renders through it).

Tauri is cross‑platform, but this POC targets **Windows** and a single `.exe`.

---

## Build & run

Install dependencies once:

```powershell
npm install
```

### Develop (hot reload)

```powershell
npm run tauri dev
```

This starts the Vite dev server on **1420** *and* launches the app against it, rebuilding on change.

### Build the single `.exe`

```powershell
npm run tauri build --no-bundle    # just the standalone binary
# or: npm run tauri build          # binary + platform installers under target/release/bundle/
```

The self‑contained release binary is written to:

```
src-tauri/target/release/finance-tauri.exe
```

Double‑click it — it creates `finance.db` in `%APPDATA%\com.luisgbm.finance\`, registers the IPC
commands, and opens the window. No install, no services, no configuration.

### Run the integration test

```powershell
cd src-tauri
cargo test
```

This exercises the full SQLite‑backed domain end‑to‑end through the `db`/`service`/command layer
(register → authenticate → accounts → categories → transactions → balances → transfers → scheduled
transaction → pay → foreign‑key cascade).

> **Heads‑up — the debug `.exe` shows a blank window if run directly.** A **debug** build loads the
> frontend from `devUrl` (`http://localhost:1420`), so `target/debug/finance-tauri.exe` only renders
> while the Vite dev server is running. Use `npm run tauri dev` for development, build a self‑contained
> debug binary with `npm run tauri build --debug --no-bundle`, or just use the **release** exe (which
> embeds the built `dist/`). The Rust core and IPC commands run fine in either build.

---

## Data & storage

| What | Where |
|---|---|
| SQLite database | `%APPDATA%\com.luisgbm.finance\finance.db` (+ `-wal` / `-shm`) |
| Log file | `%LOCALAPPDATA%\com.luisgbm.finance\logs\finance.log` |
| App identifier | `com.luisgbm.finance` (from `tauri.conf.json`) |
| Frontend ↔ Rust | Tauri IPC (`invoke`) — no network, no port |

The database persists between runs and is created empty — register a user in the app on first launch.
To reset, delete the `com.luisgbm.finance` folder.

---

## See also

- [`finance/`](../finance) — the original Axum + PostgreSQL REST API this backend was ported from.
- [`financejs/`](../financejs) — the original React web app this frontend reuses.
- [monorepo README](../README.md) — the Docker‑based web stack overview and quickstart.
