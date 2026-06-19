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
    // When running in Docker on Windows (project on /mnt/c), native filesystem
    // events don't propagate into the Linux container, so fall back to polling for
    // hot reload. Enabled via VITE_USE_POLLING=true (set in docker-compose); native
    // `npm run dev` leaves it off for efficient, event-based watching.
    watch: process.env.VITE_USE_POLLING
      ? { usePolling: true, interval: 300 }
      : undefined,
  },
  envPrefix: ['VITE_', 'REACT_APP_'],
});
