// Forwards browser console warnings/errors/info to the Tauri log plugin so they are written
// to the app log file. This matters in the release build, which hides the console, so
// otherwise-invisible runtime errors (including CSP violations and uncaught exceptions) still
// get recorded. No-ops gracefully outside the Tauri webview (e.g. a plain browser dev server).
import { warn, error, info } from '@tauri-apps/plugin-log';

const toMessage = (args) =>
    args
        .map((arg) => {
            if (typeof arg === 'string') return arg;
            if (arg instanceof Error) return arg.stack || arg.message;
            try {
                return JSON.stringify(arg);
            } catch {
                return String(arg);
            }
        })
        .join(' ');

const forward = (name, logger) => {
    const original = console[name].bind(console);
    console[name] = (...args) => {
        original(...args);
        try {
            // The plugin call is async and best-effort; never let logging break the app.
            logger(toMessage(args)).catch(() => {});
        } catch {
            /* Tauri APIs unavailable — ignore. */
        }
    };
};

export const setupLogForwarding = () => {
    if (typeof window === 'undefined' || !window.__TAURI_INTERNALS__) {
        return;
    }
    forward('warn', warn);
    forward('error', error);
    forward('info', info);
};
