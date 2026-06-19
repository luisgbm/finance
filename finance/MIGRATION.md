# Finance Backend — Rust Stack Migration Report

**Project:** `finance` (REST API backend for the `financejs` React frontend)
**Migration:** Rocket 0.4 + Diesel 1.4 (nightly Rust) → **Axum 0.8 + SQLx 0.8 + Tokio (stable Rust)**
**Date:** 2026-06-16
**Version:** `1.0.2` → `2.0.0`
**Status:** ✅ Complete and verified (30/30 behavioural parity checks passing)

---

## 1. Executive summary

The `finance` backend was fully re-implemented from a 2021-era Rocket 0.4.7 / Diesel 1.4.5 stack
(which required a **pinned nightly toolchain**, `nightly-2021-12-01`) onto a modern, fully asynchronous
stack built on **Axum 0.8**, **SQLx 0.8**, and **Tokio**, running on **stable Rust** (1.96).

The migration was **behaviour-preserving**: every HTTP route, status code, JSON field name, enum
encoding, authentication scheme, and balance calculation was reproduced exactly. The existing React
frontend works against the new backend **with no changes**, and users created by the old backend
(including the `demo` account) continue to authenticate, because the PostgreSQL-side password hashing
scheme was deliberately retained.

| Metric | Before (Rocket) | After (Axum) |
|---|---|---|
| Web framework | Rocket 0.4.7 | Axum 0.8.9 |
| ORM / DB layer | Diesel 1.4.5 (sync) | SQLx 0.8.6 (async) |
| Async runtime | none (blocking) | Tokio 1.52 |
| Rust toolchain | **nightly-2021-12-01** | **stable 1.96** |
| Native build deps | libpq (`PQ_LIB_DIR`) | none (pure-Rust driver) |
| Source size | 1,673 LOC / 27 files | 1,902 LOC / 21 files |
| HTTP endpoints | 30 | 30 (identical paths) |
| Compiler warnings | — | 0 |
| Parity tests | — | 30 / 30 passing |

---

## 2. Motivation

The original stack had several structural problems:

1. **Required nightly Rust.** Rocket 0.4 relies on `#![feature(plugin, proc_macro_hygiene, decl_macro)]`,
   so it cannot build on stable. The pinned 2021-era dependencies (`ring 0.16`, `time 0.1`,
   `proc-macro2 0.4`, `syn 0.15`) do not compile on modern compilers, forcing a **specific historical
   nightly** (`1.59.0-nightly`, 2021-11-30).
2. **Synchronous I/O.** Diesel 1.4 is blocking; the server could not take advantage of async concurrency.
3. **Heavy native build dependency.** Diesel's `pq-sys` links the system `libpq`, requiring `PQ_LIB_DIR`
   and a matching PostgreSQL client library at build time.
4. **Unmaintained versions.** Rocket 0.4, Diesel 1.4, and `jsonwebtoken 7` are several major versions
   behind, with no security or compatibility updates.

The modern stack resolves all four: it builds on stable Rust, is async-first, uses a pure-Rust Postgres
driver (no `libpq`), and tracks current, maintained crate versions.

---

## 3. Stack & dependency changes

### Before (Rocket / Diesel)

| Crate | Version | Role |
|---|---|---|
| `rocket`, `rocket_codegen` | 0.4.7 | Web framework (nightly) |
| `rocket_contrib` | 0.4.7 | JSON + Diesel Postgres pool |
| `rocket_cors` | 0.5.1 | CORS |
| `diesel` | 1.4.5 | Sync ORM (postgres, chrono, r2d2) |
| `diesel-derive-enum` | 1.x | Postgres enum mapping |
| `jsonwebtoken` | 7 | JWT |
| `chrono` / `chronoutil` | 0.4 / 0.2.1 | Dates / relative durations |

### After (Axum / SQLx) — resolved versions

| Crate | Version | Role |
|---|---|---|
| `axum` | 0.8.9 | Web framework (stable, Tower/Hyper) |
| `tokio` | 1.52.3 | Async runtime |
| `tower` / `tower-http` | 0.5.3 / 0.7.0 | Middleware (CORS, tracing) |
| `sqlx` | 0.8.6 | Async DB driver (postgres, chrono, macros, rustls) |
| `jsonwebtoken` | 10.4.0 | JWT (with `rust_crypto` provider) |
| `serde` / `serde_json` | 1.0.228 / 1.0.150 | (de)serialization |
| `chrono` / `chronoutil` | 0.4.45 / 0.2.7 | Dates / relative durations |
| `tracing` / `tracing-subscriber` | 0.1.44 / 0.3 | Structured logging |
| `thiserror` / `anyhow` | 2.0.18 / 1 | Error types |
| `dotenvy` | 0.15.7 | `.env` loading |

Transitively notable: `hyper 1.10`, `rustls 0.23`, `ring 0.17` (the latter requires a C toolchain at
build time — see §9).

---

## 4. Architecture changes

The original layout mixed Rocket route guards, "controller" helpers, and Diesel query modules. The new
layout is a clean, conventional Axum + repository structure.

### Before
```
src/
  main.rs                 # rocket::ignite().mount(...).launch()
  routes/                 # #[get]/#[post] handlers + db_pool + auth_guard
  controllers/            # cross-cutting helpers (balance, joins)
  database/               # Diesel: schema.rs, models.rs, per-table query fns
  utils/                  # jwt, next-date math
```

### After
```
src/
  main.rs                 # tokio runtime, PgPool, router, CORS/trace, axum::serve
  config.rs               # env-driven Config
  state.rs                # AppState { PgPool, Arc<Config> }
  error.rs                # AppError + IntoResponse + From<sqlx::Error>
  auth.rs                 # Claims, JWT encode/validate, AuthUser extractor
  models.rs               # enums + DB rows + request/response DTOs
  service.rs              # balance, joins, scheduled enrichment, next-date math
  db/                     # SQLx query modules (users, categories, accounts,
                          #   transactions, transfers, scheduled_transactions)
  handlers/               # Axum HTTP handlers, one module per resource
```

**Layering:** `handlers` (HTTP) → `service` (business logic) → `db` (SQL). Shared state (`PgPool` +
`Config`) is injected via `axum::extract::State`. Authentication is an `AuthUser` extractor
(`FromRequestParts`) that replaces Rocket's `Authentication` request guard.

---

## 5. Key technical decisions

### 5.1 Web framework → Axum
Chosen over Actix-web and Rocket 0.5 for its ergonomics, async-first design, and the Tower/Hyper
middleware ecosystem. Routes are composed per-resource as `Router<AppState>` and merged in `main.rs`.
Axum 0.8's `matchit` router cleanly supports static + dynamic routes on the same path (e.g.
`/api/categories/expense` alongside `/api/categories/{id}`), which the original relied on via Rocket
route ranking.

### 5.2 Database → SQLx (async, runtime queries)
SQLx provides an async, pure-Rust Postgres driver with a built-in connection pool (`PgPool`,
`max_connections = 20`, matching the original). **Runtime queries** (`query_as`, `query_scalar`) were
chosen over the compile-time-checked `query!` macros so the build does **not** require a live database
or an offline `.sqlx` cache — important for CI and first-clone builds. Custom Postgres enum types are
mapped with `#[derive(sqlx::Type)]`.

> **Version note:** SQLx is pinned to **0.8.6**, not 0.9.0. SQLx 0.9 introduced a `SqlSafeStr` guard that
> only accepts `&'static str`, rejecting the dynamically-built (`format!`) query strings used here. Moving
> to 0.9 would require converting queries to literal SQL or wrapping with `AssertSqlSafe(...)`. 0.8.6 is
> the current stable line and is fully async/modern.

### 5.3 Password hashing — **retained pgcrypto bcrypt** (compatibility)
The modern default would be Argon2, but the existing database stores bcrypt hashes produced by
PostgreSQL's `pgcrypto` (`crypt(password, gen_salt('bf', rounds))`). To avoid invalidating existing
users, the new backend keeps the identical SQL scheme:

```sql
-- insert
INSERT INTO app_users (name, password)
VALUES ($1, crypt($2, gen_salt('bf', $3)))   -- $3 = BF_ROUNDS
-- authenticate
SELECT ... FROM app_users WHERE name = $1 AND password = crypt($2, password)
```

This preserves the `BF_ROUNDS` configuration knob and was verified: the `demo` user created by the old
backend still logs in. *(A future Argon2 migration could be done lazily on next successful login.)*

### 5.4 JWT — claims kept byte-compatible
The `Claims` struct (`user_id`, `iat`, `exp`) and its numeric-timestamp serialization are reproduced
exactly, so tokens issued by the old backend remain valid under the same `JWT_SECRET`. `Validation` is
configured for `HS256` with `validate_aud = false` (the tokens carry no `aud`).

### 5.5 jsonwebtoken 10 — explicit crypto provider
jsonwebtoken 10 made its crypto backend **opt-in**; its `default` feature is only `use_pem`, so with no
provider selected it **panics at the first encode/decode** ("Could not automatically determine the
process-level CryptoProvider"). Resolved by enabling the **`rust_crypto`** feature (pure-Rust
HMAC/SHA2/RSA), which—unlike `aws_lc_rs`—needs no CMake/NASM/C toolchain on Windows:

```toml
jsonwebtoken = { version = "10", features = ["rust_crypto"] }
```

The v10 `encode`/`decode`/`Validation` API is identical to v9, so no code changes were required.

### 5.6 Enum encoding — dual representation
The JSON API uses **PascalCase** enum values (e.g. `"Expense"`, `"TransferIncome"`, `"Transaction"`,
`"Days"`) while the Postgres enum types use **snake_case**. These are kept independent:

```rust
#[derive(sqlx::Type, Serialize, Deserialize, ...)]
#[sqlx(type_name = "category_types", rename_all = "snake_case")] // DB: expense, transfer_income
pub enum CategoryTypes { Expense, Income, TransferIncome, TransferExpense } // JSON: PascalCase (serde default)
```

(The scheduled-kind Postgres type name `scheduled_transacion_kinds` retains the original schema's
spelling.)

### 5.7 Errors — `AppError` + `IntoResponse`
A single `AppError` enum maps to the exact HTTP status codes the frontend expects
(`401`, `404`, `409`, `400`, `500`). `From<sqlx::Error>` centralises DB-error translation (e.g.
unique-violation → `409 Conflict` on duplicate user name; `RowNotFound` → `404`). This replaces the
original's scattered `match` on Diesel errors and `.expect()` panics.

### 5.8 Balance calculation — pushed into SQL
The original accumulated balances in Rust by loading every transaction and transfer. The new backend
computes the same result in a single aggregate query (income − expense − transfers-out + transfers-in),
which is both clearer and more efficient while producing identical values.

### 5.9 Other
- **CORS:** `tower_http::cors::CorsLayer::permissive()` mirrors the original `rocket_cors` default
  (the API authenticates via Bearer token, not cookies, so credentials are not needed).
- **Config:** environment-driven via `dotenvy` + a `.env` file. `ROCKET_DATABASES=...` is replaced by a
  standard `DATABASE_URL`.
- **Logging:** `tracing` + `tracing-subscriber` with `TraceLayer` replace Rocket's built-in logging.

---

## 6. Configuration changes

| Concern | Before | After |
|---|---|---|
| DB connection | `ROCKET_DATABASES={finance_db={url=...,pool_size=20}}` | `DATABASE_URL=postgres://finance:finance@localhost:5432/financedb` |
| Environment | `ROCKET_ENV=development` | `BIND_ADDR` / `PORT` (default `127.0.0.1:8000`) |
| JWT | `JWT_SECRET`, `JWT_VALIDITY_DAYS` | unchanged |
| Hashing | `BF_ROUNDS` | unchanged |
| Loading | shell env vars | `.env` file (via `dotenvy`) + shell env |

---

## 7. API surface (unchanged — 30 endpoints)

All paths, methods, request/response shapes, and status codes are preserved.

| Group | Endpoints |
|---|---|
| Auth (3) | `POST /api/login`, `GET /api/token`, `POST /api/users` |
| Categories (7) | `POST/GET /api/categories`, `GET /api/categories/expense`, `GET /api/categories/income`, `GET/PATCH/DELETE /api/categories/{id}` |
| Accounts (5) | `POST/GET /api/accounts`, `GET/PATCH/DELETE /api/accounts/{id}` |
| Transactions (5) | `POST/GET /api/transactions/account/{account_id}`, `GET/PATCH/DELETE /api/transactions/{id}` |
| Transfers (4) | `POST /api/transfers/from/{origin}/to/{destination}`, `GET/PATCH/DELETE /api/transfers/{id}` |
| Scheduled (6) | `POST/GET /api/scheduled-transactions`, `GET/PATCH/DELETE /api/scheduled-transactions/{id}`, `POST /api/scheduled-transactions/{id}/pay` |

---

## 8. Verification & behavioural parity

A 30-assertion parity suite was run against the live server and database:

- **Auth:** register → token, duplicate user → `409`, bad login → `401`, missing auth → `401`,
  token refresh.
- **Enums:** PascalCase echo for categories, `TransferExpense` / `TransferIncome` in joined views,
  `Transaction` scheduled kind.
- **Status codes:** `404` for missing resources, `400` for invalid scheduled payloads.
- **Balance math:** `+income − expense ∓ transfers` verified across accounts (e.g. +1000 − 300 = 700;
  −200 transfer → 500/+200).
- **Joined transaction/transfer view:** correct count, `from_account_name`, and date-desc ordering.
- **Scheduled lifecycle:** create → enrich → pay → delete; balance effects confirmed.

**Result: 30 / 30 passing.** Additional confirmations:

- The pre-existing `demo` user (hashed by the **old** backend) logs in successfully → password-hash
  compatibility.
- The `financejs` frontend (`http://localhost:3000`) works unchanged against the new backend.
- Clean compile: **0 warnings**.

---

## 9. Toolchain & infrastructure

- **Rust:** moved from pinned `nightly-2021-12-01` to **stable** (1.96).
- **C toolchain:** the MSVC "Desktop development with C++" workload is required to build `ring 0.17`
  (used by `rustls`/SQLx TLS). This replaces the previous `libpq`/`PQ_LIB_DIR` requirement — SQLx's
  Postgres driver is pure Rust, so `libpq` is no longer needed at build or run time.
- **Database:** PostgreSQL 16.4 (the schema and existing data are unchanged; the original Diesel
  migrations under `migrations/` still describe the schema and were not re-run).

---

## 10. Known limitations & recommended follow-ups

1. **SQLx 0.9 upgrade** — convert dynamic query strings to literal SQL (or `AssertSqlSafe`) to adopt the
   `SqlSafeStr` safety guard, then bump to 0.9.x.
2. **Compile-time-checked queries** — optionally adopt `sqlx::query!`/`query_as!` with an offline
   `.sqlx` cache (`cargo sqlx prepare`) for compile-time SQL validation.
3. **Argon2 password hashing** — migrate hashes lazily on login while keeping pgcrypto verification as a
   fallback for not-yet-migrated users.
4. **JWT secret** — `.env` ships a development placeholder `JWT_SECRET`; set a strong secret in any real
   deployment.
5. **OpenAPI** — consider `utoipa` to generate API docs from the handlers.
6. **Integration tests** — fold the parity suite into a committed test harness (`#[tokio::test]` +
   `sqlx::test`).

---

## 11. Build & run

The backend now runs via **Docker Compose** from the repository root (`docker compose up --build`),
which builds the Rust image (multi‑stage → slim Debian) and starts PostgreSQL with the schema applied
automatically. See the [backend README](./README.md#build--run) and the
[monorepo README](../README.md) for details. Configuration is supplied by the `backend` service in
`docker-compose.yml` (`DATABASE_URL`, `JWT_SECRET`, `BIND_ADDR=0.0.0.0`, `PORT`, …).

---

## 12. Appendix — new source files

| File | LOC | Purpose |
|---|---:|---|
| `src/main.rs` | 58 | Runtime, pool, router, CORS/trace, `axum::serve` |
| `src/config.rs` | 35 | Env-driven configuration |
| `src/state.rs` | 9 | Shared `AppState` |
| `src/error.rs` | 50 | `AppError` + `IntoResponse` + sqlx mapping |
| `src/auth.rs` | 94 | JWT + `AuthUser` extractor |
| `src/models.rs` | 271 | Enums, DB rows, request/response DTOs |
| `src/service.rs` | 203 | Balance, joins, scheduled enrichment, next-date |
| `src/db/*.rs` | 542 | SQLx query modules (6 resources + mod) |
| `src/handlers/*.rs` | 640 | Axum handlers (6 resources + mod) |
| **Total** | **1,902** | 21 files |
