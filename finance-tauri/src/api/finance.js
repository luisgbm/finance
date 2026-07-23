import { invoke } from '@tauri-apps/api/core';

// Thin adapter over Tauri's `invoke`, shaped to look like the axios responses the rest of the
// app was written against. On success it resolves to `{ data }`; on failure it throws an Error
// whose `.response = { status, data }` mirrors an axios error, so the existing
// `err.response.status` checks throughout the components keep working unchanged.
//
// Rust commands return `Result<T, AppError>`; a rejected command surfaces here as the
// serialized error object `{ status, message }`.
export async function invokeApi(command, args = {}) {
    try {
        const data = await invoke(command, args);
        return { data };
    } catch (err) {
        const status = err && typeof err.status === 'number' ? err.status : 500;
        const message = err && err.message ? err.message : String(err);
        const wrapped = new Error(message);
        wrapped.response = { status, data: { error: message } };
        throw wrapped;
    }
}
