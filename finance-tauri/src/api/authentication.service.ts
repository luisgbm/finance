import { commands, call, clearSession } from './finance';

// The opaque session token minted by the backend at login/register, persisted in localStorage
// under 'token'. It replaces the previous scheme of sending the raw user id: the token is
// unguessable and resolved to a user id server-side, so the WebView can no longer read another
// user's data by changing a number. Persisting it keeps the user logged in across restarts.
const getToken = () => localStorage.getItem('token');

const newUser = async (name: string, password: string) => {
    try {
        const { data } = await call(commands.register({ name, password }));
        localStorage.setItem('token', data.token);
        return data;
    } catch (e) {
        logout();
        throw e;
    }
};

const login = async (name: string, password: string) => {
    try {
        const { data } = await call(commands.login({ name, password }));
        localStorage.setItem('token', data.token);
        return data;
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
    clearSession();
    if (token != null) {
        commands.logout(token).catch(() => {});
    }
};

const validateToken = async () => {
    try {
        const token = getToken();
        if (token == null) {
            return null;
        }
        return await call(commands.getInitialData(token));
    } catch {
        return null;
    }
};

export const authenticationService = {
    login,
    logout,
    newUser,
    validateToken,
};
