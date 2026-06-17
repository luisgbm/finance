# Finance — Frontend (Web App)

The **frontend** of Finance, a personal money manager. It is a single‑page **React** application
(built with **Vite**) that talks to the [`finance`](../finance) REST API. From here you create
accounts and categories, record transactions and transfers, and plan recurring scheduled
transactions — and the app shows your balances and history.

> Migrated from React 17 + Material‑UI v4 + react‑router v5 (Create React App) to React 19 + MUI v9 +
> react‑router v7 (Vite). See [`MIGRATION.md`](./MIGRATION.md) for the full migration report.

---

## How to use the app

Finance lets you keep track of your money. A few things need to be set up first.

### Categories

The first thing to do after logging in is to create a few categories.

Categories are of two types: **Expenses** and **Incomes**. Expenses are things you spend money on,
like *Food* or your *Car*. Incomes are things you earn money from, like the *Salary* from your job,
or *Earnings* from the stock market.

### Accounts

Transactions live inside accounts, so next create a few accounts. These can be your *Bank account*,
your *Credit Card*, or a *Wallet* for cash. Each account shows its **balance**, computed from all of
its transactions and transfers.

### Transactions

With categories and accounts in place, you can create a transaction. A transaction has a **value**, a
**category**, a **date**, and an optional **description**. An **Expense** transaction subtracts money
from the account; an **Income** transaction adds money to it.

### Transfers

Transfers are a special kind of transaction. They have no category, but they have an origin account
(**From**) and a destination account (**To**), plus a value, a date, and an optional description. A
transfer subtracts money from the origin account and adds it to the destination account. In an
account's history, transfers appear alongside transactions (marked as transfer in/out).

### Scheduled transactions

Scheduled transactions are planned movements — a future **transaction** or **transfer**. They can be
**one‑off** or **recurring** (every N days/weeks/months/years, for a fixed number of repeats or
indefinitely). The bottom navigation shows a badge with how many are **due**. When you **pay** a
scheduled item, the app creates the real transaction/transfer; one‑off (or finished) items are then
removed, while recurring ones advance to their next date.

> Tip: amounts are entered in your currency (e.g. `15.00`) and stored as integer cents by the API.

---

## Tech stack

- **Framework:** [React](https://react.dev/) 19
- **Build tool:** [Vite](https://vitejs.dev/) 8
- **UI:** [MUI](https://mui.com/) 9 (Material UI) + `@mui/x-date-pickers` + Emotion
- **Routing:** react‑router‑dom 7
- **State:** Redux Toolkit + react‑redux, persisted to `localStorage` via redux‑persist
- **Forms/validation:** Formik + Yup
- **HTTP:** axios · **dates:** moment · **money:** currency.js

### Architecture

```
index.html              # Vite entry -> /src/main.jsx
vite.config.js          # React plugin, dev server on port 3000, REACT_APP_ env prefix
src/
  main.jsx              # createRoot + Redux Provider + PersistGate
  components/           # screens & UI (App.jsx holds the theme + routes)
    accounts/ categories/ transactions/ scheduled-transactions/ users/
    App.jsx BottomNavBar.jsx LoadingModal.jsx MessageModal.jsx Settings.jsx
  redux/                # store (configureStore) + reducers/slices
  api/                  # axios instance + one service per resource
  context/              # loading & message modal contexts
  utils/                # money formatting helpers
```

The app holds the JWT in `localStorage` and sends it as `Authorization: Bearer <token>` on every
request; a `401` redirects to the login screen. The MUI theme in `App.jsx` is configured to match the
original Material‑UI v4 look.

---

## Prerequisites

- **Node.js** 18+ (tested on Node 25) and npm.
- The **backend API** must be running and reachable (see [`../finance`](../finance)). By default the
  app targets `http://localhost:8000/api`.

---

## Setup

```bash
cd financejs
npm install
```

Configuration is read from env files via Vite (the `REACT_APP_` prefix is kept for compatibility):

- `.env.development` → `REACT_APP_API_BASE_URL=http://localhost:8000/api`
- `.env.production` → your deployed API URL

Adjust these if your backend runs elsewhere.

---

## Run

```bash
npm run dev       # start the Vite dev server at http://localhost:3000
npm run build     # production build to dist/
npm run preview   # serve the production build locally
```

Open **http://localhost:3000**, then register a user or log in. Make sure the backend is running
first, otherwise login and data requests will fail.

---

## See also

- [`finance`](../finance) — the Rust/Axum REST API this app consumes.
- [`MIGRATION.md`](./MIGRATION.md) — CRA/React 17/MUI v4 → Vite/React 19/MUI v9 migration report.
