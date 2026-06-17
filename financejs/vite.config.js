import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Dev server stays on port 3000 to match the original setup, and `REACT_APP_` env vars
// remain exposed so `.env.development` / `.env.production` (and the backend base URL) are
// unchanged. Components live in `.jsx` files so Rolldown/oxc applies the JSX transform.
export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
    host: true,
  },
  envPrefix: ['VITE_', 'REACT_APP_'],
});
