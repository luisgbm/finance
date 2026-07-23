import finance from './finance';

const getWithAuth = async (url) => {
    try {
        return await finance.get(url, authenticationService.getAuthHeader());
    } catch (e) {
        if (e.response.status === 401) logout();
        throw(e);
    }
}

const postWithAuth = async (url, body) => {
    try {
        return await finance.post(url, body, authenticationService.getAuthHeader());
    } catch (e) {
        if (e.response.status === 401) logout();
        throw(e);
    }
}

const patchWithAuth = async (url, body) => {
    try {
        return await finance.patch(url, body, authenticationService.getAuthHeader());
    } catch (e) {
        if (e.response.status === 401) logout();
        throw(e);
    }
}

const deleteWithAuth = async (url) => {
    try {
        return await finance.delete(url, authenticationService.getAuthHeader());
    } catch (e) {
        if (e.response.status === 401) logout();
        throw(e);
    }
}

const newUser = async (name, password) => {
    try {
        const result = await finance.post('/users', {
            name,
            password
        });

        localStorage.setItem('token', result.data.token);

        return result.data;
    } catch (e) {
        logout();
        throw(e);
    }
};

const login = async (name, password) => {
    try {
        const result = await finance.post('/login', {
            name,
            password
        });

        localStorage.setItem('token', result.data.token);

        return result.data;
    } catch (e) {
        logout();
        throw(e);
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

        return await getWithAuth('/token');
    } catch {
        return null;
    }
};

const getAuthHeader = () => {
    return {
        headers: {
            'Authorization': `Bearer ${localStorage.getItem('token')}`
        }
    };
}

export const authenticationService = {
    login,
    logout,
    newUser,
    getWithAuth,
    postWithAuth,
    patchWithAuth,
    deleteWithAuth,
    validateToken,
    getAuthHeader
};