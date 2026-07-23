import {authenticationService} from "./authentication.service";

const getAllTransactionsForAccountId = async (accountId) => {
    try {
        return await authenticationService.getWithAuth(`/transactions/account/${accountId}`);
    } catch (e) {
        console.log(e);
        throw(e);
    }
};

const getTransactionById = async (transactionId) => {
    try {
        return await authenticationService.getWithAuth(`/transactions/${transactionId}`);
    } catch (e) {
        throw(e);
    }
};

const newTransaction = async (accountId, value, description, date, category) => {
    try {
        return await authenticationService.postWithAuth(`/transactions/account/${accountId}`, {
            value,
            description,
            date,
            category
        });
    } catch (e) {
        throw(e);
    }
};

const editTransactionById = async (transactionId, value, description, date, account, category) => {
    try {
        return await authenticationService.patchWithAuth(`/transactions/${transactionId}`, {
            value,
            description,
            date,
            account,
            category
        });
    } catch (e) {
        throw(e);
    }
};

const deleteTransactionById = async (transactionId) => {
    try {
        return await authenticationService.deleteWithAuth(`/transactions/${transactionId}`);
    } catch (e) {
        throw(e);
    }
};

export const transactionService = {
    getAllTransactionsForAccountId,
    getTransactionById,
    newTransaction,
    editTransactionById,
    deleteTransactionById
};
