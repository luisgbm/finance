import {authenticationService} from "./authentication.service";

const getAllAccounts = async () => {
    const accounts = await authenticationService.callAuthed('get_accounts');
    return accounts.data;
};

const getAccountById = async (accountId) => {
    return await authenticationService.callAuthed('get_account', {accountId});
};

const newAccount = async (name) => {
    const account = await authenticationService.callAuthed('create_account', {name});
    return account.data;
};

const editAccountById = async (accountId, name) => {
    const account = await authenticationService.callAuthed('update_account', {accountId, name});
    return account.data;
};

const deleteAccountById = async (accountId) => {
    return await authenticationService.callAuthed('delete_account', {accountId});
};

export const accountService = {
    getAllAccounts,
    getAccountById,
    newAccount,
    editAccountById,
    deleteAccountById
};
