# Finance Frontend — React Stack Migration Report

**Project:** `financejs` (React SPA for the `finance` REST API)
**Migration:** React 17 + Material‑UI v4 + react‑router v5 (Create React App) → **React 19 + MUI v9 + react‑router v7 (Vite)**
**Date:** 2026-06-16
**Version:** `1.0.3` → `2.0.0`
**Status:** ✅ Complete and verified (headless E2E: 8/8 functional + 5/5 theme‑parity checks passing, 0 console errors)

---

## 1. Executive summary

The `financejs` frontend was fully migrated from a 2021‑era stack — React 17, Material‑UI **v4** (JSS `makeStyles`), react‑router **v5**, and **Create React App** (react‑scripts 4, webpack 4) — to a modern stack built on **React 19**, **MUI v9** (Emotion + `sx`), **react‑router v7**, **Redux Toolkit**, and **Vite 8**.

The migration is **behaviour‑ and layout‑preserving**: every screen, route, label, and interaction is unchanged, and **communication with the backend is byte‑identical** (same `/api/*` calls, payloads, headers, and the `yyyy-MM-DDTHH:mm:ss` date format). After the component migration, the MUI theme was configured to replicate the Material‑UI v4 default palette and component defaults so the visual appearance matches the original.

| Metric | Before (CRA) | After (Vite) |
|---|---|---|
| React | 17.0.1 | 19.2.7 |
| UI library | Material‑UI 4.11 (`makeStyles`/JSS) | MUI 9.1 (`sx`/Emotion) |
| Date pickers | `@material-ui/pickers` 3 + `@date-io/moment` 1 | `@mui/x-date-pickers` 9.5 |
| Router | react‑router‑dom 5.2 | react‑router‑dom 7.18 |
| State | redux 4 + redux‑thunk + devtools | Redux Toolkit 2.12 |
| Forms validation | yup 0.32 | yup 1.7 |
| Build tool | react‑scripts 4 (webpack 4) | Vite 8 (Rolldown) |
| Source size | 4,571 LOC / 57 files | 4,350 LOC / 56 files |
| Routes | 20 | 20 (identical) |
| Verification | — | 8/8 functional + 5/5 theme, 0 console errors |

---

## 2. Motivation

The original stack was several major versions behind and increasingly hard to run:

1. **Deprecated build tool.** Create React App (react‑scripts) is unmaintained; on modern Node it only runs with the `--openssl-legacy-provider` workaround (webpack 4 + OpenSSL 3 incompatibility).
2. **Unmaintained UI library.** Material‑UI v4 uses JSS (`makeStyles`) and does not support React 18/19. Its successor (MUI v5→v9) switched to Emotion and removed the JSS styling engine.
3. **Outdated React & router.** React 17 predates the modern root API and concurrent features; react‑router v5 uses a render‑prop API replaced by hooks in v6/v7.
4. **Aging ecosystem.** redux `createStore`, redux‑thunk, and `redux-devtools-extension` are superseded by Redux Toolkit; yup 0.x and axios 0.x are several majors behind.

The new stack runs on stable, current libraries, builds far faster (Vite), and is fully React‑19 compatible.

---

## 3. Stack & dependency changes

### Before (CRA / Material‑UI v4)

| Package | Version | Role |
|---|---|---|
| `react`, `react-dom` | 17.0.1 | UI runtime |
| `@material-ui/core` `/icons` `/styles` | 4.11 | UI components + JSS styling |
| `@material-ui/pickers` + `@date-io/moment` | 3.2 / 1.3 | Date/time pickers |
| `react-router-dom` | 5.2 | Routing |
| `redux` + `react-redux` + `redux-thunk` | 4 / 7.2 / 2.3 | State |
| `redux-devtools-extension` | 2.13 | Devtools |
| `formik` / `yup` | 2.2 / 0.32 | Forms / validation |
| `axios` | 0.21 | HTTP |
| `react-scripts` | 4.0.1 | Build (webpack 4) |

### After (Vite / MUI v9) — resolved versions

| Package | Version | Role |
|---|---|---|
| `react`, `react-dom` | 19.2.7 | UI runtime |
| `@mui/material`, `@mui/icons-material` | 9.1.1 | UI components (Emotion) |
| `@mui/x-date-pickers` | 9.5.0 | Date/time pickers |
| `@emotion/react`, `@emotion/styled` | 11.14.0 | Styling engine |
| `react-router-dom` | 7.18.0 | Routing |
| `@reduxjs/toolkit` + `react-redux` | 2.12.0 / 9.3.0 | State |
| `redux-persist` | 6.0.0 | Persistence (kept) |
| `formik` / `yup` | 2.4.9 / 1.7.1 | Forms / validation |
| `axios` | 1.18.0 | HTTP |
| `moment` / `currency.js` | 2.30.1 / 2.0.4 | Dates / money (kept) |
| `vite` + `@vitejs/plugin-react` | 8.0.16 / 6.0.2 | Build |

Removed: `react-scripts`, `redux`, `redux-thunk`, `redux-devtools-extension`, `@date-io/moment`, `@material-ui/*`, `web-vitals`, `@testing-library/*`.

---

## 4. Architecture changes

The component tree, Redux slices, API service layer, and React contexts were preserved. The structural changes are confined to the build entry, the store wiring, and per‑component styling/router APIs.

### Before
```
public/index.html          # CRA HTML template (%PUBLIC_URL%)
src/index.js               # ReactDOM.render(...)
src/reportWebVitals.js
src/components/**/*.js      # JSX in .js, makeStyles, route-prop components
src/redux/{store,reducer,*Slice}.js   # createStore + thunk + devtools
src/api/**, src/context/**, src/utils/**
```

### After
```
index.html                 # Vite entry at project root -> /src/main.jsx
vite.config.js             # @vitejs/plugin-react, port 3000, envPrefix
src/main.jsx               # createRoot(...).render(...)
src/components/**/*.jsx     # JSX in .jsx, sx styling, hook-based routing
src/redux/store.js         # configureStore + inline localStorage adapter
src/api/**, src/context/**, src/utils/**   # unchanged
```

JSX‑containing files were renamed `.js` → `.jsx` (34 files); imports are extensionless so resolution is unaffected. Pure‑JS modules (services, slices, contexts, constants, `*FormParams`) stay `.js`.

---

## 5. Key technical decisions

### 5.1 Build tool → Vite (over react‑scripts 5)
Create React App is deprecated; Vite is the modern standard, builds in ~1.7s, and needs no OpenSSL workaround on Node 25. The dev server is pinned to **port 3000** (`vite.config.js`) to match the original.

### 5.2 JSX in `.js` → rename to `.jsx`
Vite 8 uses **Rolldown/oxc**, which does **not** transform JSX inside `.js` files and **ignores** the legacy `esbuild` loader config. The robust fix is to rename JSX files to `.jsx` (imports are extensionless, so nothing else changes).

### 5.3 Styling: `makeStyles` → `sx`
`@mui/styles` (the v4 JSS engine) is not available for MUI v9, so all 33 `makeStyles` usages were converted to the `sx` prop (`theme.spacing(3)` → `mb: 3`, `flexGrow: 1`, theme callbacks for `zIndex`, etc.). Styles applied to non‑MUI elements (raw `<span>`) use inline `style`.

### 5.4 Routing: react‑router v5 → v7
`Switch`/`Route component=` → `Routes`/`Route element=`; `props.history`/`props.match` → `useNavigate`/`useParams`. Child form components receive a `navigate` prop instead of `history`.

### 5.5 State: Redux Toolkit + an inline storage adapter
`createStore` + `redux-thunk` + `redux-devtools-extension` → a single `configureStore` (thunk + devtools built in). The plain reducers/slices and string action types are unchanged. **redux‑persist** is retained, but `import storage from 'redux-persist/lib/storage'` resolves to the module namespace under Vite ("storage.getItem is not a function"), so an **inline `localStorage` adapter** is used instead — identical behaviour (persists under the `finance` key).

> RTK freezes state in development, so components that sorted a `useSelector` result in place now copy first (`[...accounts].sort(...)`).

### 5.6 Date pickers
`@material-ui/pickers` + `@date-io/moment` → `@mui/x-date-pickers` with a single root `LocalizationProvider`/`AdapterMoment` in `App.jsx`. Pickers store the raw `moment` value on change and use `slotProps={{ textField: { variant, fullWidth, error, helperText } }}`. The display format uses the standard token `DD/MM/YYYY HH:mm` (MUI X renders unrecognized lowercase `yyyy` literally). Submission still uses `moment(date).format('yyyy-MM-DDTHH:mm:ss')` — unchanged.

### 5.7 yup 0.x → 1.x
`.when('field', { is, then: yup.x().required() })` → `.when('field', { is, then: (schema) => schema.required() })` (function form), including nested conditions, in the `*FormParams` files.

### 5.8 Theme: replicate the Material‑UI v4 default
MUI changed many palette and component defaults in v5+. To keep the original look app‑wide, `createTheme` in `App.jsx` reproduces the v4 theme:
- **Palette** — primary `#3f51b5`, secondary `#f50057`, error `#f44336`, warning `#ff9800`, info `#2196f3`, success `#4caf50` (each with v4 `light`/`dark`/`contrastText`).
- **`MuiChip`** — opaque grey `#e0e0e0` background with dark text (v5+ made it a translucent overlay, invisible on the indigo app bar).
- **`MuiTabs`** — `textColor: 'inherit'`, `indicatorColor: 'secondary'` (v5+ defaulted to `primary`, invisible on the app bar).

---

## 6. Configuration changes

| Concern | Before | After |
|---|---|---|
| Build/run | `react-scripts start` (+`--openssl-legacy-provider`) | `vite` (`npm run dev`) on port 3000 |
| HTML entry | `public/index.html` | root `index.html` → `/src/main.jsx` |
| Root render | `ReactDOM.render` | `createRoot(...).render` |
| API base URL | `process.env.REACT_APP_API_BASE_URL` | `import.meta.env.REACT_APP_API_BASE_URL` |
| Env files | `.env.development` / `.env.production` | **unchanged** (Vite `envPrefix: ['VITE_', 'REACT_APP_']`) |

The `.env.development` (`http://localhost:8000/api`) and `.env.production` (Azure host) files are untouched — backend targeting is identical.

---

## 7. Routes & backend communication (unchanged)

All 20 routes and the entire API contract are preserved. Routes include: `/` (login), `/settings`, `/users/new`, `/accounts` (+ `/new`, `/edit/:id`), `/categories/` (+ `/:type`, `/new/:type`, `/edit/:id`), `/transactions/account/:accountId` (+ `/new/:type`), `/transactions/:transactionId`, `/transfers/:transferId/from/:fromAccountId`, and the `/scheduled-transactions` / `/scheduled-transfers` set (list, new, edit, pay).

**Backend communication is byte‑identical**: the axios instance, every service call, request payloads (including money in integer cents via `currency().intValue`), the `Authorization: Bearer` header, and the `yyyy-MM-DDTHH:mm:ss` submission date format are all unchanged.

---

## 8. Verification & parity

Automated headless testing (Microsoft Edge via Puppeteer) against the live backend:

**Functional (8/8, 0 console errors)** — register/seed via API, then drive the UI: login → `/accounts` (account card + computed balance `$3,800.00`) → transactions list (income + expense) → edit transaction (date picker shows the formatted date, all fields populated, Delete button present).

**Theme parity (5/5)** — app bar `rgb(63,81,181)` (v4 indigo), Logout/Delete button `rgb(245,0,87)` (v4 red), selected category tab white & visible, balance chip `rgb(224,224,224)` opaque grey pill with dark text.

The production build (`vite build`) succeeds with 0 warnings (aside from a bundle‑size hint).

---

## 9. Toolchain & infrastructure

- **Node:** runs on Node 25 with no flags (Vite). The CRA `--openssl-legacy-provider` workaround is gone.
- **Dev server:** `vite` on `http://localhost:3000` (run via Docker Compose).
- **Fonts:** `@fontsource/roboto` v5 (per‑weight CSS imports: 300/400/500/700).
- **No backend changes:** the frontend talks to the same Axum API on `:8000`.

---

## 10. Known limitations & recommended follow-ups

1. **MUI `Switch` look** — the v4 `Switch` (used in the scheduled‑transaction repeat options) has a different built‑in appearance in v9; it was not pixel‑matched (not user‑visible as a regression). Can be restored via `MuiSwitch` style overrides if desired.
2. **Bundle size** — a single ~990 kB chunk; could be reduced with route‑level `React.lazy`/dynamic imports and `build.rolldownOptions` code‑splitting.
3. **Tests** — the original `@testing-library` setup was dropped; the Puppeteer smoke/theme suites could be committed as an automated check (e.g. Playwright).
4. **State modernization (optional)** — the plain reducers + string action types still work under RTK; they could be converted to `createSlice` for less boilerplate.
5. **redux‑persist** — somewhat unmaintained; the inline storage adapter is a pragmatic shim.

---

## 11. Build & run

The app now runs via **Docker Compose** from the repository root (`docker compose up --build`); the
Vite dev server (with HMR) and its npm scripts run **inside** the container, and the backend + database
come up alongside it. See the [frontend README](./README.md#run) and the
[monorepo README](../README.md) for details. The browser‑facing API URL is configured via
`REACT_APP_API_BASE_URL` (from the root `.env`'s `API_BASE_URL`).

---

## 12. Appendix — source layout

| Area | Files | Notes |
|---|---|---|
| Build/entry | `index.html`, `vite.config.js`, `src/main.jsx` | Vite + createRoot |
| App shell | `src/components/App.jsx` | theme (v4 replica), routes, LocalizationProvider |
| Components | `src/components/**/*.jsx` (~40) | accounts, categories, transactions, scheduled‑transactions, users, modals, nav |
| State | `src/redux/{store,reducer,accountsSlice,categoriesSlice,scheduledTransactionsSlice}.js` | RTK store + plain reducers |
| Services | `src/api/*.js` (7) | axios — unchanged |
| Contexts / utils / constants | `src/context/*`, `src/utils/*`, `CategoryTypes.js`, `RepeatFrequencies.js`, `*FormParams.js`, `due.scheduled.transactions.js` | unchanged (yup `.when` updated in params) |
| **Total** | **4,350 LOC / 56 files** | |
