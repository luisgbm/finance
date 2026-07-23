# Finance — Desktop app (Tauri POC)

A **proof‑of‑concept** that repackages Finance as a **completely local desktop app**, distributed as
a **single `.exe`** with no server, no Docker, and no external database. It wraps the existing React
frontend and the original Axum REST API in a [**Tauri 2**](https://tauri.app) shell and swaps
**PostgreSQL for embedded SQLite**, so the whole personal money manager runs offline from one file.

> This is an experiment that lives alongside the production monorepo. The
> [`finance/`](../finance) (Axum + PostgreSQL API) and [`financejs/`](../financejs) (React web app)
> projects remain the canonical apps; this directory reuses their code almost verbatim. See the
> [monorepo README](../README.md) for the Docker‑based web stack.

---

## How it works

Everything runs **in one process**. There is no separate backend to start — the Rust side embeds the
full REST API and talks to SQLite in‑process:

1. On launch, the Rust `setup()` resolves a writable SQLite path in the OS **app‑data directory**
   (`%APPDATA%\com.luisgbm.finance\finance.db` on Windows) and creates the schema if missing.
2. It starts the original **Axum** API on an **ephemeral loopback port** (`127.0.0.1:0` → the OS
   picks a free port), served by the same Tokio runtime Tauri already uses.
3. It creates the **WebView2** window and injects `window.__FINANCE_API_BASE__ =
   'http://127.0.0.1:<port>/api'` as an initialization script — *before* any page script runs — so
   the reused React app points its axios client at the local API.
4. The React SPA then behaves exactly like the web app, only every `/api` call goes to the embedded
   server instead of a remote one.

```
                         finance-tauri.exe  (one process)
   +----------------------------------------------------------------------+
   |                                                                      |
   |   WebView2 window                     Embedded backend (Axum)        |
   |   +------------------------+          +------------------------+     |
   |   | React SPA (MUI/Vite)   |          | REST API  /api/...     |     |
   |   |                        | -------> | (handlers -> service   |     |
   |   | axios baseURL =        |  HTTP    |  -> db)                |     |
   |   | window.__FINANCE_      |  JSON    |                        |     |
   |   |   API_BASE__           | <------- |     |  SQLx (bundled)  |     |
   |   +------------------------+          +-----|------------------+     |
   |                                             v                        |
   |                                       finance.db  (SQLite file,      |
   |                                       in %APPDATA%\com.luisgbm.finance)
   +----------------------------------------------------------------------+

   No network server, no Docker, no PostgreSQL. The API listens only on 127.0.0.1
   on an OS-assigned port; the database is a single local SQLite file.
```

The domain (accounts, categories, transactions, transfers, scheduled transactions, computed
balances, integer‑cents money) is unchanged — see [`finance/README.md`](../finance/README.md) for the
full model.

---

## What changed vs. the web app

The port was deliberately minimal. The **frontend** is the `financejs` React app reused almost
verbatim; the only functional change is [`src/api/finance.js`](./src/api/finance.js) (reads the
injected `window.__FINANCE_API_BASE__`, falling back to the Vite env var), plus a Tauri‑tuned
[`vite.config.js`](./vite.config.js) (`base: './'`, fixed port `1420`, `REACT_APP_` env prefix).

The **backend** is the original Axum API with a PostgreSQL → SQLite translation:

| Original (Postgres) | Desktop POC (SQLite) |
|---|---|
| `PgPool` | `SqlitePool` (bundled SQLite, compiled from source — no external DLL) |
| `$1, $2 …` placeholders | `?` placeholders |
| `pgcrypto` (`crypt`/`gen_salt`) password hashing | [`bcrypt`](https://crates.io/crates/bcrypt) crate, in‑process |
| `CREATE TYPE … ENUM` types | plain `TEXT` columns (snake_case enum names) |
| shared `transactions_transfers_id_seq` sequence | a `seq_tx_tr` table drawn from inside a transaction |
| `ON DELETE CASCADE` | same, with `PRAGMA foreign_keys = ON` per connection |
| balance via one aggregate SQL query | three `query_scalar::<i64>` sums combined in Rust |
| env‑driven config (`DATABASE_URL`, `JWT_SECRET` …) | built‑in local defaults in [`config.rs`](./src-tauri/src/config.rs); a fixed local JWT secret + long validity so a persisted login survives restarts |

The SQLite schema lives in [`src-tauri/src/schema.sql`](./src-tauri/src/schema.sql) and is embedded
into the binary (`include_str!`) and applied idempotently on every launch.

---

## Tech stack

- **Shell:** [Tauri 2](https://tauri.app) (WebView2 on Windows) — window, packaging, app‑data paths
- **Frontend:** React 19 · MUI 9 · Vite 8 (the `financejs` app, reused)
- **Backend (embedded):** [Axum](https://github.com/tokio-rs/axum) 0.8 on Tokio
- **Database:** [SQLx](https://github.com/launchbadge/sqlx) 0.8 with the **bundled `sqlite`** driver + local `finance.db`
- **Auth:** `jsonwebtoken` 10 (HS256, pure‑Rust) + `bcrypt` 0.15
- **Other:** `serde`, `chrono`/`chronoutil`, `tower-http` (CORS/tracing), `tracing`, `thiserror`, `anyhow`

---

## Project layout

```
finance-tauri/
├── index.html            # Vite entry for the React app
├── vite.config.js        # Tauri-tuned Vite config (base './', port 1420)
├── package.json          # frontend deps + the `tauri` script
├── public/               # static assets (favicon, logos, manifest)
├── src/                  # the reused React SPA (only src/api/finance.js changed)
│   ├── api/              # axios services; finance.js resolves the injected API base URL
│   ├── components/       # accounts, categories, transactions, transfers, scheduled, users
│   ├── redux/  context/  utils/
│   └── main.jsx
└── src-tauri/            # the Rust/Tauri side
    ├── Cargo.toml        # crate deps + release profile (LTO, strip, panic=abort)
    ├── tauri.conf.json   # identifier, devUrl/frontendDist, bundle config
    ├── build.rs          # tauri-build
    ├── capabilities/     # Tauri capability/permission set
    ├── icons/            # generated app icons (from app-icon.png)
    ├── src/
    │   ├── main.rs       # binary entry (hides console in release) -> lib::run()
    │   ├── lib.rs        # Tauri setup(): start backend, inject API base URL, build window
    │   ├── bootstrap.rs  # build_router() + start(): open SQLite, apply schema, serve Axum
    │   ├── schema.sql    # embedded SQLite DDL
    │   ├── config.rs state.rs error.rs auth.rs models.rs service.rs
    │   ├── db/           # SQLx query modules (users, accounts, categories, …)
    │   └── handlers/     # Axum handlers, one module per resource
    └── tests/
        └── api.rs        # end-to-end integration test over the SQLite-backed API
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

Double‑click it — it creates `finance.db` in `%APPDATA%\com.luisgbm.finance\`, starts the embedded
API on a loopback port, and opens the window. No install, no services, no configuration.

### Run the API integration test

```powershell
cd src-tauri
cargo test --test api
```

This exercises the full SQLite‑backed API end‑to‑end (register → JWT → accounts → transactions → …).

> **Heads‑up — the debug `.exe` shows a blank window if run directly.** A **debug** build loads the
> frontend from `devUrl` (`http://localhost:1420`), so `target/debug/finance-tauri.exe` only renders
> while the Vite dev server is running. Use `npm run tauri dev` for development, build a self‑contained
> debug binary with `npm run tauri build --debug --no-bundle`, or just use the **release** exe (which
> embeds the built `dist/`). The embedded backend itself runs fine in either build.

---

## Data & storage

| What | Where |
|---|---|
| SQLite database | `%APPDATA%\com.luisgbm.finance\finance.db` (+ `-wal` / `-shm`) |
| App identifier | `com.luisgbm.finance` (from `tauri.conf.json`) |
| API address | `http://127.0.0.1:<ephemeral-port>/api` (loopback only) |

The database persists between runs and is created empty — register a user in the app on first launch.
To reset, delete the `com.luisgbm.finance` folder.

---

## See also

- [`finance/`](../finance) — the original Axum + PostgreSQL REST API this backend was ported from.
- [`financejs/`](../financejs) — the original React web app this frontend reuses.
- [monorepo README](../README.md) — the Docker‑based web stack overview and quickstart.
