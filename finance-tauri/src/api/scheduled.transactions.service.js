import {authenticationService} from "./authentication.service";

const getAllScheduledTransactions = async () => {
    const scheduledTransactions = await authenticationService.callAuthed('get_scheduled_transactions');
    return scheduledTransactions.data;
};

const getScheduledTransactionById = async (scheduledTransactionId) => {
    const scheduledTransaction = await authenticationService.callAuthed('get_scheduled_transaction', {scheduledTransactionId});
    return scheduledTransaction.data;
};

const newScheduledTransaction = async (kind, accountId, value, description, categoryId, originAccountId, destinationAccountId, createdDate, repeat, repeatFreq, repeatInterval, infiniteRepeat, endAfterRepeats) => {
    const scheduledTransaction = await authenticationService.callAuthed('create_scheduled_transaction', {
        req: {
            kind,
            account_id: accountId,
            value,
            description,
            category_id: categoryId,
            origin_account_id: originAccountId,
            destination_account_id: destinationAccountId,
            created_date: createdDate,
            repeat,
            repeat_freq: repeat ? repeatFreq : null,
            repeat_interval: repeat ? repeatInterval : null,
            infinite_repeat: repeat ? infiniteRepeat : null,
            end_after_repeats: repeat ? (infiniteRepeat ? null : endAfterRepeats) : null
        }
    });

    return scheduledTransaction.data;
};

const payScheduledTransaction = async (scheduledTransactionId, value, description, date, categoryId, accountId, originAccountId, destinationAccountId) => {
    const transaction = await authenticationService.callAuthed('pay_scheduled_transaction', {
        scheduledTransactionId,
        req: {
            value,
            description,
            date,
            category_id: categoryId,
            account_id: accountId,
            origin_account_id: originAccountId,
            destination_account_id: destinationAccountId
        }
    });

    return transaction.data;
};

const editScheduledTransactionById = async (scheduledTransactionId, kind, accountId, value, description, categoryId, originAccountId, destinationAccountId, createdDate, repeat, repeatFreq, repeatInterval, infiniteRepeat, endAfterRepeats) => {
    const scheduledTransaction = await authenticationService.callAuthed('update_scheduled_transaction', {
        scheduledTransactionId,
        req: {
            kind,
            account_id: accountId,
            value,
            description,
            category_id: categoryId,
            origin_account_id: originAccountId,
            destination_account_id: destinationAccountId,
            created_date: createdDate,
            repeat,
            repeat_freq: repeat ? repeatFreq : null,
            repeat_interval: repeat ? repeatInterval : null,
            infinite_repeat: repeat ? infiniteRepeat : null,
            end_after_repeats: repeat ? (infiniteRepeat ? null : endAfterRepeats) : null
        }
    });

    return scheduledTransaction.data;
};

const deleteScheduledTransactionById = async (transactionId) => {
    const scheduledTransaction = await authenticationService.callAuthed('delete_scheduled_transaction', {
        scheduledTransactionId: transactionId
    });
    return scheduledTransaction.data;
};

export const scheduledTransactionService = {
    getAllScheduledTransactions,
    getScheduledTransactionById,
    newScheduledTransaction,
    payScheduledTransaction,
    editScheduledTransactionById,
    deleteScheduledTransactionById
};
