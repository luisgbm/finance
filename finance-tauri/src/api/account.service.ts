import { commands, call } from './finance';

const getAllAccounts = async () => {
    const { data } = await call(commands.getAccounts());
    return data;
};

const getAccountById = async (accountId: number) => {
    return await call(commands.getAccount(accountId));
};

const newAccount = async (name: string) => {
    const { data } = await call(commands.createAccount(name));
    return data;
};

const editAccountById = async (accountId: number, name: string) => {
    const { data } = await call(commands.updateAccount(accountId, name));
    return data;
};

const deleteAccountById = async (accountId: number) => {
    return await call(commands.deleteAccount(accountId));
};

export const accountService = {
    getAllAccounts,
    getAccountById,
    newAccount,
    editAccountById,
    deleteAccountById,
};
