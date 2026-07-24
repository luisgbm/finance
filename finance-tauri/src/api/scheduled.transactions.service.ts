import { commands, call } from './finance';
import type { ScheduledTransactionKinds, RepeatFrequencies } from './bindings';

const getAllScheduledTransactions = async () => {
    const { data } = await call(commands.getScheduledTransactions());
    return data;
};

const getScheduledTransactionById = async (scheduledTransactionId: number) => {
    const { data } = await call(commands.getScheduledTransaction(scheduledTransactionId));
    return data;
};

const newScheduledTransaction = async (
    kind: ScheduledTransactionKinds,
    accountId: number | null,
    value: number,
    description: string | null,
    categoryId: number | null,
    originAccountId: number | null,
    destinationAccountId: number | null,
    createdDate: string,
    repeat: boolean,
    repeatFreq: RepeatFrequencies | null,
    repeatInterval: number | null,
    infiniteRepeat: boolean | null,
    endAfterRepeats: number | null,
) => {
    const { data } = await call(
        commands.createScheduledTransaction({
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
            end_after_repeats: repeat ? (infiniteRepeat ? null : endAfterRepeats) : null,
            // Server-computed on create; sent explicitly as null (equivalent to omitting them)
            // so the object matches the shared PostScheduledTransaction shape.
            current_repeat_count: null,
            next_date: null,
        }),
    );

    return data;
};

const payScheduledTransaction = async (
    scheduledTransactionId: number,
    value: number,
    description: string,
    date: string,
    categoryId: number | null,
    accountId: number | null,
    originAccountId: number | null,
    destinationAccountId: number | null,
) => {
    const { data } = await call(
        commands.payScheduledTransaction(scheduledTransactionId, {
            value,
            description,
            date,
            category_id: categoryId,
            account_id: accountId,
            origin_account_id: originAccountId,
            destination_account_id: destinationAccountId,
        }),
    );

    return data;
};

const editScheduledTransactionById = async (
    scheduledTransactionId: number,
    kind: ScheduledTransactionKinds,
    accountId: number | null,
    value: number,
    description: string | null,
    categoryId: number | null,
    originAccountId: number | null,
    destinationAccountId: number | null,
    createdDate: string,
    repeat: boolean,
    repeatFreq: RepeatFrequencies | null,
    repeatInterval: number | null,
    infiniteRepeat: boolean | null,
    endAfterRepeats: number | null,
) => {
    const { data } = await call(
        commands.updateScheduledTransaction(scheduledTransactionId, {
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
            end_after_repeats: repeat ? (infiniteRepeat ? null : endAfterRepeats) : null,
            current_repeat_count: null,
            next_date: null,
        }),
    );

    return data;
};

const deleteScheduledTransactionById = async (transactionId: number) => {
    const { data } = await call(commands.deleteScheduledTransaction(transactionId));
    return data;
};

export const scheduledTransactionService = {
    getAllScheduledTransactions,
    getScheduledTransactionById,
    newScheduledTransaction,
    payScheduledTransaction,
    editScheduledTransactionById,
    deleteScheduledTransactionById,
};
