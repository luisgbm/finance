import {authenticationService} from "./authentication.service";

const getAllAccounts = async () => {
    const accounts = await authenticationService.getWithAuth('/accounts');
    return accounts.data;
};

const getAccountById = async (accountId) => {
    try {
        return await authenticationService.getWithAuth(`/accounts/${accountId}`);
    } catch (e) {
        throw(e);
    }
};

const newAccount = async (name) => {
    const account = await authenticationService.postWithAuth('/accounts', {
        name
    });

    return account.data;
};

const editAccountById = async (accountId, name) => {
    const account = await authenticationService.patchWithAuth(`/accounts/${accountId}`, {
        name
    });

    return account.data;
};

const deleteAccountById = async (accountId) => {
    return await authenticationService.deleteWithAuth(`/accounts/${accountId}`);
};

export const accountService = {
    getAllAccounts,
    getAccountById,
    newAccount,
    editAccountById,
    deleteAccountById
};
