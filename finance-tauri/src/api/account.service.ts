import { commands, call, requireToken } from './finance';

const getAllAccounts = async () => {
    const { data } = await call(commands.getAccounts(requireToken()));
    return data;
};

const getAccountById = async (accountId: number) => {
    return await call(commands.getAccount(requireToken(), accountId));
};

const newAccount = async (name: string) => {
    const { data } = await call(commands.createAccount(requireToken(), name));
    return data;
};

const editAccountById = async (accountId: number, name: string) => {
    const { data } = await call(commands.updateAccount(requireToken(), accountId, name));
    return data;
};

const deleteAccountById = async (accountId: number) => {
    return await call(commands.deleteAccount(requireToken(), accountId));
};

export const accountService = {
    getAllAccounts,
    getAccountById,
    newAccount,
    editAccountById,
    deleteAccountById,
};
