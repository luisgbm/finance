import {authenticationService} from "./authentication.service";

const newTransfer = async (value, description, from, to, date) => {
    try {
        return await authenticationService.callAuthed('create_transfer', {
            originAccount: from,
            destinationAccount: to,
            req: {value, description, date}
        });
    } catch (e) {
        throw(e);
    }
};

const getTransferById = async (transferId) => {
    try {
        return await authenticationService.callAuthed('get_transfer', {transferId});
    } catch (e) {
        throw(e);
    }
};

const deleteTransferById = async (transferId) => {
    try {
        return await authenticationService.callAuthed('delete_transfer', {transferId});
    } catch (e) {
        throw(e);
    }
};

const editTransferById = async (transferId, value, description, date, from, to) => {
    try {
        return await authenticationService.callAuthed('update_transfer', {
            transferId,
            req: {
                origin_account: from,
                destination_account: to,
                value,
                description,
                date
            }
        });
    } catch (e) {
        throw(e);
    }
};

export const transferService = {
    newTransfer,
    getTransferById,
    deleteTransferById,
    editTransferById
};
