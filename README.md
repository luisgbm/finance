# Finance

**Finance** is a personal money manager: track your accounts, categorize income and
expenses, record transactions and transfers, and plan recurring scheduled transactions.

This is a **monorepo** containing both halves of the web application, plus an experimental
desktop build:

| Path | Component | Stack |
|---|---|---|
| [`finance/`](./finance) | Backend — REST API | Rust · Axum · SQLx · PostgreSQL |
| [`financejs/`](./financejs) | Frontend — Web app (SPA) | React 19 · MUI 9 · Vite |
| [`finance-tauri/`](./finance-tauri) | Desktop app (POC) — single local `.exe` | Tauri 2 · React 19 · Axum · SQLite |

Each subproject has its own detailed `README.md` (and the two web projects a `MIGRATION.md`). This
root README is the shared overview and **quickstart** to get the web stack running; the desktop POC
has its own build instructions in [`finance-tauri/README.md`](./finance-tauri/README.md).

---

## How it works (the domain)

- **Accounts** — money containers (Bank, Credit Card, Wallet…). Balances are computed, not stored.
- **Categories** — labels of type **Expense** or **Income** (Food, Salary…).
- **Transactions** — a value + category + date in an account. Income adds to the balance, Expense subtracts.
- **Transfers** — move money between two of your accounts (no category): subtract from origin, add to destination.
- **Scheduled transactions** — planned, optionally recurring transactions/transfers you can "pay" when due.

An account's balance is `Σ(income) − Σ(expense) − Σ(transfers out) + Σ(transfers in)`. All money is
stored as **integer cents**. See [`finance/README.md`](./finance/README.md) and
[`financejs/README.md`](./financejs/README.md) for the full details.

```
Browser ──HTTP/JSON──> financejs (React/Vite :3000) ──/api──> finance (Axum :8000) ──> PostgreSQL :5432
```

---

## Prerequisites

The whole stack runs in Docker — the only thing you need installed is the container engine:

- **Docker Engine + Compose v2** (or Docker Desktop). *(On this machine, Docker runs inside
  **WSL**; invoke it as `wsl docker compose …` or from a WSL shell in the project directory.)*

No local Rust, Node, or PostgreSQL install is required — Compose builds and runs all three.

---

## Quickstart

One command builds everything, starts PostgreSQL, applies the schema, then starts the API and the
web app:

```bash
cp .env.example .env        # set JWT_SECRET (Windows: copy .env.example .env)
docker compose up --build   # WSL: wsl docker compose up --build
```

Then open **http://localhost:3000**, register a user, and start adding categories, accounts, and
transactions. The API is on **http://localhost:8000**.

To stop: `Ctrl+C`, or `docker compose down` (add `-v` to also drop the database volume).

### What Compose runs

Three services on a private bridge network, started in dependency order
(**db** becomes healthy → **backend** → **frontend**):

```
                        Docker host (your machine)

   Browser  ->  localhost:3000      (web app UI)
            ->  localhost:8000/api  (API calls, sent to the host port)

          3000                   8000                   5432   (published ports)
            |                      |                      |
   =========|======================|======================|=======  network "finance"
            v                      v                      v
   +-----------------+    +-----------------+    +-----------------+
   | frontend (Vite) | -> | backend (Axum)  | -> | db (postgres)   |
   | React + HMR     |    | REST API        |    | financedb       |
   | :3000           |    | :8000           |    | :5432 (pgdata)  |
   +-----------------+    +-----------------+    +-----------------+

   Startup order:  db (healthy)  ->  backend  ->  frontend

   The browser calls the API on the published host port (localhost:8000), not the
   internal name. Inside the network the backend reaches Postgres as "db:5432"
   (DATABASE_URL); the database persists in the pgdata volume.
```

| Service | Image / build | Host port | Notes |
|---|---|---|---|
| `db` | `postgres:16` | `5432` | Creates `finance`/`financedb`; auto‑applies the schema from the mounted migration on first boot; `pg_isready` healthcheck; data persisted in the `pgdata` volume. |
| `backend` | built from [`finance/Dockerfile`](./finance/Dockerfile) | `8000` | Multi‑stage Rust → slim Debian image; `BIND_ADDR=0.0.0.0`; connects to `db`. Waits for the DB healthcheck. |
| `frontend` | built from [`financejs/Dockerfile`](./financejs/Dockerfile) | `3000` | Vite dev server with **hot reload** (source bind‑mounted; container keeps its own Linux `node_modules`). |

### Configuration & customization

Compose reads `./.env` (copy it from [`.env.example`](./.env.example)). All values have safe
defaults:

| Variable | Default | Purpose |
|---|---|---|
| `JWT_SECRET` | `dev-secret-change-me` | Backend JWT signing secret — **change it** for anything but local dev. |
| `DB_PORT` / `BACKEND_PORT` / `FRONTEND_PORT` | `5432` / `8000` / `3000` | Host ports. Override if any are already in use, e.g. `5433` / `8001` / `3001` (then update `API_BASE_URL` to match). |
| `API_BASE_URL` | `http://localhost:8000/api` | Browser‑facing API URL — must match `BACKEND_PORT`. |

On the **first** boot the database is created **empty** (schema only) — register a user via the app
or the API. Your data then lives in the `pgdata` volume and persists across `docker compose up` /
`down`, so you only register once (it is cleared only by `docker compose down -v`). The first build
compiles the Rust release binary and installs npm packages, so it takes a few minutes; later runs are
cached, and code changes hot‑reload (frontend) or rebuild with `docker compose up --build` (backend).

> **WSL note:** with Docker running inside WSL (not Docker Desktop), the WSL VM — and therefore the
> containers — stops when WSL goes idle. Keep a WSL shell/`docker compose up` running, or rely on the
> services' `restart: unless-stopped` policy to bring them back when the daemon restarts.
>
> **Port note:** make sure nothing else on the host is already using `3000`, `8000`, or `5432`
> (a previously‑running local Postgres or dev server, for example) before starting the stack; if so,
> override the `*_PORT` values above (and `API_BASE_URL`).

---

## Repository layout

```
.
├── finance/             # Rust/Axum REST API   (own README.md + MIGRATION.md + Dockerfile)
├── financejs/           # React/Vite web app   (own README.md + MIGRATION.md + Dockerfile)
├── finance-tauri/       # Tauri desktop POC    (own README.md; single-exe, embedded SQLite)
├── docker-compose.yml   # full dev stack: db + backend + frontend
├── .env.example         # compose configuration template (copy to .env)
└── README.md            # you are here (shared overview + quickstart)
```

History for both subprojects is preserved under their respective directories
(e.g. `git log -- finance/`, `git log -- financejs/`).

---

## Documentation

- Backend details & API: [`finance/README.md`](./finance/README.md)
- Frontend details & usage: [`financejs/README.md`](./financejs/README.md)
- Desktop app (Tauri POC): [`finance-tauri/README.md`](./finance-tauri/README.md)
- Migration reports: [`finance/MIGRATION.md`](./finance/MIGRATION.md), [`financejs/MIGRATION.md`](./financejs/MIGRATION.md)
