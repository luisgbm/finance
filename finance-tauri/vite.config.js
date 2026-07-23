import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Vite config for the Tauri desktop build.
// - `base: './'` makes the built asset URLs relative so they resolve correctly when
//   Tauri serves the bundle from the app (tauri://localhost / http://tauri.localhost).
// - The dev server runs on a fixed port that matches `build.devUrl` in tauri.conf.json.
// - `clearScreen: false` keeps Rust/cargo output visible during `tauri dev`.
export default defineConfig({
  plugins: [react()],
  base: './',
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: true,
  },
  // Keep the `REACT_APP_` prefix exposed so the existing frontend code (which reads
  // import.meta.env.REACT_APP_API_BASE_URL as a fallback) keeps working unchanged.
  envPrefix: ['VITE_', 'REACT_APP_'],
  build: {
    // A broadly-compatible target for the bundled WebView2 (Chromium) runtime.
    target: 'es2021',
    outDir: 'dist',
    emptyOutDir: true,
  },
});
