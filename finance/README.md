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

The backend runs as a container in the monorepo's Docker Compose stack — you do **not** need a local
Rust toolchain, C compiler, or PostgreSQL to build or run it.

- **Docker Engine + Compose v2** (or Docker Desktop). *(On this machine, Docker runs inside **WSL**.)*

Everything else (the Rust release build, the Postgres database, the schema) is handled by Compose.
See the [monorepo README](../README.md) for the one‑command quickstart.

> SQLx uses a pure‑Rust Postgres driver and **runtime** queries (no compile‑time DB access and no
> libpq), which is what keeps the Docker build fully self‑contained.

---

## Database

You don't set up the database manually. The Compose `db` service runs `postgres:16`, creates the
`finance` role and `financedb` database, and **auto‑applies the schema** on first boot by mounting
[`migrations/2021-01-19-171757_finance/up.sql`](./migrations/2021-01-19-171757_finance/up.sql) into
the Postgres init directory — it creates the `pgcrypto` extension, the enums, and all tables. Your
data then persists in the `pgdata` volume between `docker compose up` / `down` runs (cleared only by
`docker compose down -v`).

The `migrations/00000000000000_diesel_initial_setup` folder is a legacy Diesel artifact and is unused.

---

## Configuration

The backend reads all configuration from **environment variables** (a `.env` file is also honored via
`dotenvy`). In Docker these are set by the `backend` service in
[`docker-compose.yml`](../docker-compose.yml); the secret and host ports come from the root `.env`
(copied from `.env.example`):

```yaml
# docker-compose.yml → services.backend.environment
DATABASE_URL: postgres://finance:finance@db:5432/financedb
BIND_ADDR: 0.0.0.0          # listen on all interfaces inside the container
PORT: 8000
JWT_SECRET: ${JWT_SECRET}   # from the root .env
JWT_VALIDITY_DAYS: 30
BF_ROUNDS: 10
RUST_LOG: finance=debug,tower_http=info,info
```

| Variable | Required | Default | Description |
|---|---|---|---|
| `DATABASE_URL` | yes | — | Postgres connection string (in Compose: `…@db:5432/financedb`) |
| `JWT_SECRET` | yes | — | HMAC secret for signing JWTs |
| `JWT_VALIDITY_DAYS` | no | `30` | Token lifetime in days |
| `BF_ROUNDS` | no | `10` | bcrypt cost for password hashing |
| `BIND_ADDR` | no | `127.0.0.1` | Listen address (Compose sets `0.0.0.0`) |
| `PORT` | no | `8000` | Listen port |
| `DB_POOL_SIZE` | no | `20` | Max DB pool connections |

---

## Build & run

The backend is built and started by Compose together with its database — run from the **repository
root**:

```bash
docker compose up --build        # WSL: wsl docker compose up --build
```

This builds the image from [`Dockerfile`](./Dockerfile) (a multi‑stage Rust → slim‑Debian build) and
serves the API on **http://localhost:8000**. After changing Rust code, rebuild just the backend:

```bash
docker compose up --build backend
```

Quick check once it is up:

```bash
curl -s -X POST http://localhost:8000/api/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"demo","password":"demo1234"}'
```

Follow logs or open a shell in the running container:

```bash
docker compose logs -f backend
docker compose exec backend /bin/sh
```

The frontend expects this API at `http://localhost:8000/api`
(`financejs/.env.development` → `REACT_APP_API_BASE_URL`).

---

## See also

- [`financejs`](../financejs) — the React frontend client.
- [`MIGRATION.md`](./MIGRATION.md) — Rocket/Diesel → Axum/SQLx migration report.
