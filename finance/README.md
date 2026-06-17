# Finance — Backend (REST API)

The **backend** of Finance, a personal money manager. It is a stateless JSON REST API written in
**Rust** with **Axum** and **SQLx**, backed by **PostgreSQL**. It owns all business logic and data:
users, accounts, categories, transactions, transfers, and scheduled transactions. The
[`financejs`](../financejs) React app is a client of this API.

> Migrated from Rocket 0.4 + Diesel 1.4 (nightly Rust) to Axum + SQLx (stable Rust). See
> [`MIGRATION.md`](./MIGRATION.md) for the full migration report.

---

## What it does

The API stores a user's finances and computes account balances on demand. Authentication is via
**JWT** (HS256 bearer tokens); passwords are hashed in the database with PostgreSQL's `pgcrypto`
(`crypt`/`gen_salt('bf', rounds)`). **All monetary values are stored as integer cents** (e.g.
`1500` = $15.00).

### Domain model

| Concept | Table | Description |
|---|---|---|
| **User** | `app_users` | An account holder (`name`, bcrypt `password`). Every other entity belongs to a user. |
| **Category** | `categories` | A label of type **Expense** or **Income** (e.g. *Food*, *Salary*). The `category_types` enum also has internal `transfer_income` / `transfer_expense` values used to render transfers. |
| **Account** | `accounts` | A money container (Bank, Credit Card, Wallet…). Has no stored balance — it is computed. |
| **Transaction** | `transactions` | A `value`, `category`, `date`, optional `description`, in one account. An **Income** category adds to the balance; an **Expense** subtracts. |
| **Transfer** | `transfers` | Moves money between two of the user's accounts (`origin_account` → `destination_account`). No category. Subtracts from the origin, adds to the destination. |
| **Scheduled transaction** | `scheduled_transactions` | A planned transaction **or** transfer (`kind` enum), optionally recurring (`repeat_frequencies`: days/weeks/months/years, finite or infinite). "Paying" one creates the real transaction/transfer and either deletes it (one‑off / finished) or advances it to the next occurrence. |

### Balance calculation

An account balance is derived from its movements (never stored):

```
balance = Σ(income transactions) − Σ(expense transactions)
        − Σ(transfers out)       + Σ(transfers in)
```

---

## Tech stack

- **Language:** Rust (stable)
- **Web framework:** [Axum](https://github.com/tokio-rs/axum) 0.8 on Tokio + Hyper
- **Database:** [SQLx](https://github.com/launchbadge/sqlx) 0.8 (async, pure‑Rust Postgres driver) + PostgreSQL 16
- **Auth:** `jsonwebtoken` 10 (HS256) + `pgcrypto` (bcrypt)
- **Other:** `serde`, `chrono`, `tower-http` (CORS/tracing), `tracing`, `thiserror`, `dotenvy`

### Architecture

Layered: `handlers` (HTTP) → `service` (business logic) → `db` (SQLx queries).

```
src/
  main.rs        # runtime, PgPool, router, CORS/trace, axum::serve
  config.rs      # env-driven configuration
  state.rs       # shared AppState { PgPool, Config }
  error.rs       # AppError -> HTTP status mapping
  auth.rs        # JWT encode/validate + AuthUser extractor
  models.rs      # enums, DB rows, request/response DTOs
  service.rs     # balance, joins, scheduled enrichment, next-date math
  db/            # SQLx query modules (users, categories, accounts, transactions, transfers, scheduled_transactions)
  handlers/      # Axum handlers, one module per resource
```

---

## API overview

All endpoints are under `/api`. All except register/login require an `Authorization: Bearer <jwt>`
header. JSON enum values are **PascalCase** (`Expense`, `Income`, `Transaction`, `Transfer`,
`Days`…); money is in integer cents; dates are `yyyy-MM-DDTHH:mm:ss`.

| Group | Endpoints |
|---|---|
| Auth | `POST /api/users` (register), `POST /api/login`, `GET /api/token` (refresh) |
| Categories | `POST/GET /api/categories`, `GET /api/categories/{expense\|income}`, `GET/PATCH/DELETE /api/categories/{id}` |
| Accounts | `POST/GET /api/accounts`, `GET/PATCH/DELETE /api/accounts/{id}` |
| Transactions | `POST/GET /api/transactions/account/{accountId}`, `GET/PATCH/DELETE /api/transactions/{id}` |
| Transfers | `POST /api/transfers/from/{origin}/to/{destination}`, `GET/PATCH/DELETE /api/transfers/{id}` |
| Scheduled | `POST/GET /api/scheduled-transactions`, `GET/PATCH/DELETE /api/scheduled-transactions/{id}`, `POST /api/scheduled-transactions/{id}/pay` |

Login / register / token‑refresh return an `InitialData` payload (`token`, `accounts`,
`categories`, `scheduled_transactions`) so the client can hydrate in one round trip.

---

## Prerequisites

- **Rust** (stable) — install via [rustup](https://rustup.rs/).
- **A C toolchain** for building `ring` (TLS):
  - **Windows:** Visual Studio with the *Desktop development with C++* workload (provides `link.exe` + the Windows SDK).
  - **Linux/macOS:** a standard C compiler (gcc/clang).
- **PostgreSQL 16** with the `pgcrypto` extension available (ships with the standard `contrib` package).

> Note: SQLx uses a pure‑Rust Postgres driver, so **libpq is not required** at build or run time.

---

## Database setup

Create the role and database, then apply the schema:

```sql
-- as a Postgres superuser
CREATE ROLE finance LOGIN PASSWORD 'finance';
CREATE DATABASE financedb OWNER finance;
```

```bash
# apply the schema (creates the pgcrypto extension, enums, and all tables)
psql -U finance -d financedb -f migrations/2021-01-19-171757_finance/up.sql
```

The `migrations/00000000000000_diesel_initial_setup` folder is a legacy Diesel artifact and is
optional. (If `CREATE EXTENSION pgcrypto` is denied, run that one statement as a superuser first.)

---

## Configuration

Configuration is read from environment variables (a local `.env` file is loaded automatically via
`dotenvy`). Create `finance/.env`:

```dotenv
DATABASE_URL=postgres://finance:finance@localhost:5432/financedb?sslmode=disable
JWT_SECRET=change-me-to-a-long-random-secret
JWT_VALIDITY_DAYS=30
BF_ROUNDS=10
BIND_ADDR=127.0.0.1
PORT=8000
# DB_POOL_SIZE=20
RUST_LOG=finance=debug,tower_http=debug,info
```

| Variable | Required | Default | Description |
|---|---|---|---|
| `DATABASE_URL` | yes | — | Postgres connection string |
| `JWT_SECRET` | yes | — | HMAC secret for signing JWTs |
| `JWT_VALIDITY_DAYS` | no | `30` | Token lifetime in days |
| `BF_ROUNDS` | no | `10` | bcrypt cost for password hashing |
| `BIND_ADDR` | no | `127.0.0.1` | Listen address |
| `PORT` | no | `8000` | Listen port |
| `DB_POOL_SIZE` | no | `20` | Max DB pool connections |

---

## Build & run

```bash
cd finance
cargo build
cargo run        # serves http://127.0.0.1:8000
```

On **Windows**, make sure the MSVC build environment is on `PATH` (run from a *Developer PowerShell*,
or call `vcvars64.bat` first) so the linker and `ring` build succeed.

Quick check:

```bash
curl -s -X POST http://localhost:8000/api/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"demo","password":"demo1234"}'
```

The frontend expects this API at `http://localhost:8000/api`
(`financejs/.env.development` → `REACT_APP_API_BASE_URL`).

---

## See also

- [`financejs`](../financejs) — the React frontend client.
- [`MIGRATION.md`](./MIGRATION.md) — Rocket/Diesel → Axum/SQLx migration report.
