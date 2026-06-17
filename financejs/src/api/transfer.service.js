import {authenticationService} from "./authentication.service";

const newTransfer = async (value, description, from, to, date) => {
    try {
        return await authenticationService.postWithAuth(`/transfers/from/${from}/to/${to}`, {
            value,
            description,
            date
        });
    } catch (e) {
        throw(e);
    }
};

const getTransferById = async (transferId) => {
    try {
        return await authenticationService.getWithAuth(`/transfers/${transferId}`);
    } catch (e) {
        throw(e);
    }
};

const deleteTransferById = async (transferId) => {
    try {
        return await authenticationService.deleteWithAuth(`/transfers/${transferId}`);
    } catch (e) {
        throw(e);
    }
};

const editTransferById = async (transferId, value, description, date, from, to) => {
    try {
        return await authenticationService.patchWithAuth(`/transfers/${transferId}`, {
            origin_account: from,
            destination_account: to,
            value,
            description,
            date
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
