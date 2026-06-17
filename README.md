# Finance

**Finance** is a personal money manager: track your accounts, categorize income and
expenses, record transactions and transfers, and plan recurring scheduled transactions.

This is a **monorepo** containing both halves of the application:

| Path | Component | Stack |
|---|---|---|
| [`finance/`](./finance) | Backend — REST API | Rust · Axum · SQLx · PostgreSQL |
| [`financejs/`](./financejs) | Frontend — Web app (SPA) | React 19 · MUI 9 · Vite |

Each subproject has its own detailed `README.md` and `MIGRATION.md`. This root README is the
shared overview and **quickstart** to get the whole thing running.

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

- **Rust** (stable) — via [rustup](https://rustup.rs/).
- **A C toolchain** for building `ring` (TLS): on **Windows**, Visual Studio with the
  *Desktop development with C++* workload; on **Linux/macOS**, gcc/clang.
- **Node.js** 18+ (tested on 25) and **npm**.
- **PostgreSQL** 16 (with the `pgcrypto` extension, included in the standard `contrib` package).

---

## Quickstart

From the repository root.

### 1. Database

Create the role/database and apply the schema:

```sql
-- as a Postgres superuser (psql)
CREATE ROLE finance LOGIN PASSWORD 'finance';
CREATE DATABASE financedb OWNER finance;
```

```bash
psql -U finance -d financedb -f finance/migrations/2021-01-19-171757_finance/up.sql
```

### 2. Backend (API → http://localhost:8000)

```bash
cd finance
cp .env.example .env        # then edit JWT_SECRET (Windows: copy .env.example .env)
cargo run
```

On **Windows**, run from a *Developer PowerShell for VS* (or after calling `vcvars64.bat`) so the
MSVC linker and `ring` build succeed. Configuration is read from `finance/.env` — see the
[backend README](./finance/README.md#configuration).

### 3. Frontend (Web app → http://localhost:3000)

In a second terminal:

```bash
cd financejs
npm install
npm run dev
```

The app targets `http://localhost:8000/api` by default (`financejs/.env.development`).

### 4. Use it

Open **http://localhost:3000**, register a user, then create categories → accounts → transactions.

> Quick API check (no UI):
> ```bash
> curl -s -X POST http://localhost:8000/api/users \
>   -H 'Content-Type: application/json' \
>   -d '{"name":"demo","password":"demo1234"}'
> ```

---

## Repository layout

```
.
├── finance/        # Rust/Axum REST API   (own README.md + MIGRATION.md)
├── financejs/      # React/Vite web app   (own README.md + MIGRATION.md)
└── README.md       # you are here (shared overview + quickstart)
```

History for both subprojects is preserved under their respective directories
(e.g. `git log -- finance/`, `git log -- financejs/`).

---

## Documentation

- Backend details & API: [`finance/README.md`](./finance/README.md)
- Frontend details & usage: [`financejs/README.md`](./financejs/README.md)
- Migration reports: [`finance/MIGRATION.md`](./finance/MIGRATION.md), [`financejs/MIGRATION.md`](./financejs/MIGRATION.md)
