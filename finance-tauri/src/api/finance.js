import axios from 'axios';

// In the Tauri desktop build the Rust side starts a local API server on an ephemeral
// port and injects its base URL as `window.__FINANCE_API_BASE__` (via a webview
// initialization script) before any app code runs. Fall back to the Vite env var
// (used by the plain web build / dev server) and finally a sensible localhost default.
const baseURL =
	(typeof window !== 'undefined' && window.__FINANCE_API_BASE__) ||
	import.meta.env.REACT_APP_API_BASE_URL ||
	'http://127.0.0.1:8000/api';

export default axios.create({
	baseURL
});
