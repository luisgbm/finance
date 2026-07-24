import { commands, call } from './finance';

const getAllTransactionsForAccountId = async (accountId: number) => {
    try {
        return await call(commands.getTransactionsForAccount(accountId));
    } catch (e) {
        console.log(e);
        throw e;
    }
};

const getTransactionById = async (transactionId: number) => {
    return await call(commands.getTransaction(transactionId));
};

const newTransaction = async (
    accountId: number,
    value: number,
    description: string,
    date: string,
    category: number,
) => {
    return await call(
        commands.createTransaction(accountId, { value, description, date, category }),
    );
};

const editTransactionById = async (
    transactionId: number,
    value: number,
    description: string,
    date: string,
    account: number,
    category: number,
) => {
    return await call(
        commands.updateTransaction(transactionId, {
            value,
            description,
            date,
            account,
            category,
        }),
    );
};

const deleteTransactionById = async (transactionId: number) => {
    return await call(commands.deleteTransaction(transactionId));
};

export const transactionService = {
    getAllTransactionsForAccountId,
    getTransactionById,
    newTransaction,
    editTransactionById,
    deleteTransactionById,
};
