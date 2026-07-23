import { commands, type Result } from './bindings';

// Central adapter between the type-safe tauri-specta client (`commands`, generated into
// `bindings.ts` from the Rust `#[tauri::command]`s) and the axios-shaped contract the React
// components were originally written against.
//
// tauri-specta's generated commands never throw for a command `Err`: they resolve to a
// discriminated `Result<T, E>` (`{ status: 'ok', data } | { status: 'error', error }`). The
// components, however, still expect the old axios behaviour — resolve to `{ data }` on success,
// throw an error whose `.response = { status, data }` on failure — so every call is funnelled
// through `call`/`unwrap` here, leaving the components unchanged.
//
// Using the generated `commands` (instead of stringly-typed `invoke('name', {...})`) is the
// point of this layer: command names, argument names/types and return types are now checked at
// compile time against the Rust signatures via `npm run typecheck`.

export { commands };

/// The error payload every command rejects with — mirrors Rust's `AppError` serializing to
/// `{ status, message }` (see `error.rs`). Kept in sync automatically: `bindings.ts` types each
/// command's error as exactly this shape.
export type CommandError = { status: number; message: string };

/// An axios-like error. Components inspect `err.response.status` (401 -> login screen, 409 ->
/// "user already exists", etc.), so failures are re-thrown in this shape.
export interface ApiError extends Error {
    response: { status: number; data: { error: string } };
}

function apiError(status: number, message: string): ApiError {
    const err = new Error(message) as ApiError;
    err.response = { status, data: { error: message } };
    return err;
}

/// Drop the persisted session token (the full logout, including notifying the backend, lives in
/// `authentication.service`). Exported so the 401 path below and the auth service share it.
export function clearSession(): void {
    localStorage.removeItem('token');
}

/// Return the opaque session token minted at login/register, or throw a 401 when it is absent —
/// mirroring the old `callAuthed`, which logged out and rejected when no token was stored.
export function requireToken(): string {
    const token = localStorage.getItem('token');
    if (token == null) {
        clearSession();
        throw apiError(401, 'unauthorized');
    }
    return token;
}

/// Translate a resolved tauri-specta `Result` into the axios-shaped contract: `{ data }` on
/// success; on error throw an axios-like error and, for a 401, clear the now-invalid session so
/// the app falls back to the login screen (exactly what the old axios interceptor did).
function unwrap<T>(result: Result<T, CommandError>): { data: T } {
    if (result.status === 'ok') {
        return { data: result.data };
    }
    const { status, message } = result.error;
    if (status === 401) {
        clearSession();
    }
    throw apiError(status, message);
}

/// Await a generated command and normalise its outcome. A resolved `Result` is unwrapped as
/// above; a *thrown* value (a transport/serialization failure rather than a command `Err`, which
/// tauri-specta re-throws) is surfaced as a generic 500 in the same axios shape, so no component
/// ever sees a bare error without `.response`.
export async function call<T>(promise: Promise<Result<T, CommandError>>): Promise<{ data: T }> {
    let result: Result<T, CommandError>;
    try {
        result = await promise;
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        throw apiError(500, message);
    }
    return unwrap(result);
}
