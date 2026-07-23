import { invokeApi } from './finance';

// The opaque session token minted by the backend at login/register, persisted in
// localStorage under 'token'. It replaces the previous scheme of sending the raw user id:
// the token is unguessable and resolved to a user id server-side, so the WebView can no
// longer read another user's data by changing a number. Persisting it keeps the user logged
// in across app restarts (exactly as the JWT, and later the id, did before).
const getToken = () => localStorage.getItem('token');

const unauthorizedError = () => {
    const e = new Error('unauthorized');
    e.response = { status: 401, data: { error: 'unauthorized' } };
    return e;
};

// Invoke an authenticated command, injecting the current session token. Logs out (clears the
// stored token) on a 401, matching the original axios interceptor behaviour.
const callAuthed = async (command, args = {}) => {
    const token = getToken();
    if (token == null) {
        logout();
        throw unauthorizedError();
    }
    try {
        return await invokeApi(command, { token, ...args });
    } catch (e) {
        if (e.response && e.response.status === 401) {
            logout();
        }
        throw e;
    }
};

const newUser = async (name, password) => {
    try {
        const result = await invokeApi('register', { req: { name, password } });
        localStorage.setItem('token', result.data.token);
        return result.data;
    } catch (e) {
        logout();
        throw e;
    }
};

const login = async (name, password) => {
    try {
        const result = await invokeApi('login', { req: { name, password } });
        localStorage.setItem('token', result.data.token);
        return result.data;
    } catch (e) {
        logout();
        throw e;
    }
};

// Clear the local session immediately (so the UI returns to login without waiting), then
// best-effort tell the backend to invalidate the token. The backend call is fire-and-forget:
// a failure (or an already-unknown token) must not block logging out locally.
const logout = () => {
    const token = getToken();
    localStorage.removeItem('token');
    if (token != null) {
        invokeApi('logout', { token }).catch(() => {});
    }
};

const validateToken = async () => {
    try {
        if (getToken() == null) {
            return null;
        }
        return await callAuthed('get_initial_data');
    } catch {
        return null;
    }
};

export const authenticationService = {
    login,
    logout,
    newUser,
    callAuthed,
    validateToken
};

