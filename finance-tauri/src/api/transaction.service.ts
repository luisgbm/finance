import { commands, call, requireToken } from './finance';

const getAllTransactionsForAccountId = async (accountId: number) => {
    try {
        return await call(commands.getTransactionsForAccount(requireToken(), accountId));
    } catch (e) {
        console.log(e);
        throw e;
    }
};

const getTransactionById = async (transactionId: number) => {
    return await call(commands.getTransaction(requireToken(), transactionId));
};

const newTransaction = async (
    accountId: number,
    value: number,
    description: string,
    date: string,
    category: number,
) => {
    return await call(
        commands.createTransaction(requireToken(), accountId, { value, description, date, category }),
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
        commands.updateTransaction(requireToken(), transactionId, {
            value,
            description,
            date,
            account,
            category,
        }),
    );
};

const deleteTransactionById = async (transactionId: number) => {
    return await call(commands.deleteTransaction(requireToken(), transactionId));
};

export const transactionService = {
    getAllTransactionsForAccountId,
    getTransactionById,
    newTransaction,
    editTransactionById,
    deleteTransactionById,
};
