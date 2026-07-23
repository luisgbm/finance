const initialState = [];

const accountsReducer = (state = initialState, action) => {
    switch (action.type) {
        case 'addAccount': {
            return [
                ...state,
                action.payload
            ].sort((a, b) => a.name.localeCompare(b.name));
        }
        case 'setAccounts': {
            return action.payload.sort((a, b) => a.name.localeCompare(b.name));
        }
        case 'editAccount': {
            return state.map((account, index) => {
                if (account.id === parseInt(action.payload.id)) {
                    return action.payload;
                }

                return account;
            }).sort((a, b) => a.name.localeCompare(b.name));
        }
        case 'deleteAccount': {
            return state.filter((account, index) => account.id !== action.payload).sort((a, b) => a.name.localeCompare(b.name));
        }
        default:
            return state;
    }
};

export {
    accountsReducer
}