import { invokeApi } from './finance';

// The logged-in user's id, persisted (as a string) in localStorage under 'token'. Since the
// JWT was dropped in the IPC migration, this id is sent as the `user_id` argument on every
// authenticated command; persisting it keeps the user logged in across app restarts, exactly
// as the JWT did before.
const getUserId = () => {
    const stored = localStorage.getItem('token');
    if (stored == null) {
        return null;
    }
    const id = parseInt(stored, 10);
    return Number.isNaN(id) ? null : id;
};

const unauthorizedError = () => {
    const e = new Error('unauthorized');
    e.response = { status: 401, data: { error: 'unauthorized' } };
    return e;
};

// Invoke an authenticated command, injecting the current user's id. Logs out (clears the
// stored id) on a 401, matching the original axios interceptor behaviour.
const callAuthed = async (command, args = {}) => {
    const userId = getUserId();
    if (userId == null) {
        logout();
        throw unauthorizedError();
    }
    try {
        return await invokeApi(command, { userId, ...args });
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

const logout = () => {
    localStorage.removeItem('token');
};

const validateToken = async () => {
    try {
        if (localStorage.getItem('token') == null) {
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
