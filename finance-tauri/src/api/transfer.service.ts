import { commands, call, requireToken } from './finance';

const newTransfer = async (
    value: number,
    description: string,
    from: number,
    to: number,
    date: string,
) => {
    return await call(commands.createTransfer(requireToken(), from, to, { value, description, date }));
};

const getTransferById = async (transferId: number) => {
    return await call(commands.getTransfer(requireToken(), transferId));
};

const deleteTransferById = async (transferId: number) => {
    return await call(commands.deleteTransfer(requireToken(), transferId));
};

const editTransferById = async (
    transferId: number,
    value: number,
    description: string,
    date: string,
    from: number,
    to: number,
) => {
    return await call(
        commands.updateTransfer(requireToken(), transferId, {
            origin_account: from,
            destination_account: to,
            value,
            description,
            date,
        }),
    );
};

export const transferService = {
    newTransfer,
    getTransferById,
    deleteTransferById,
    editTransferById,
};
