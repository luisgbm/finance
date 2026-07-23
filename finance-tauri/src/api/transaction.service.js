import {authenticationService} from "./authentication.service";

const getAllTransactionsForAccountId = async (accountId) => {
    try {
        return await authenticationService.callAuthed('get_transactions_for_account', {accountId});
    } catch (e) {
        console.log(e);
        throw(e);
    }
};

const getTransactionById = async (transactionId) => {
    try {
        return await authenticationService.callAuthed('get_transaction', {transactionId});
    } catch (e) {
        throw(e);
    }
};

const newTransaction = async (accountId, value, description, date, category) => {
    try {
        return await authenticationService.callAuthed('create_transaction', {
            accountId,
            req: {value, description, date, category}
        });
    } catch (e) {
        throw(e);
    }
};

const editTransactionById = async (transactionId, value, description, date, account, category) => {
    try {
        return await authenticationService.callAuthed('update_transaction', {
            transactionId,
            req: {value, description, date, account, category}
        });
    } catch (e) {
        throw(e);
    }
};

const deleteTransactionById = async (transactionId) => {
    try {
        return await authenticationService.callAuthed('delete_transaction', {transactionId});
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
