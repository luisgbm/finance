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
    rollupOptions: {
      output: {
        // Split the large third-party libraries out of the app entry chunk. For a desktop
        // app the bundle is loaded from local disk (not the network), so this is about
        // keeping individual chunks reasonable and cacheable rather than transfer size.
        // rolldown-vite requires the function form; order matters so that e.g. react-redux
        // is grouped with the state libs rather than caught by the generic `react` check.
        manualChunks: (id) => {
          if (!id.includes('node_modules')) return undefined;
          if (id.includes('@mui') || id.includes('@emotion')) return 'mui';
          if (
            id.includes('@reduxjs') ||
            id.includes('react-redux') ||
            id.includes('redux-persist')
          )
            return 'state';
          if (
            id.includes('formik') ||
            id.includes('yup') ||
            id.includes('moment') ||
            id.includes('currency.js')
          )
            return 'forms';
          if (id.includes('react') || id.includes('scheduler')) return 'react';
          return 'vendor';
        },
      },
    },
    // The assets are served from disk by the WebView, so the network-oriented default
    // 500 kB chunk-size warning does not apply here; raise it to avoid noisy build output.
    chunkSizeWarningLimit: 1500,
  },
});
