const initialState = [];

const scheduledTransactionsReducer = (state = initialState, action) => {
    switch (action.type) {
        case 'addScheduledTransaction': {
            return [
                ...state,
                action.payload
            ];
        }
        case 'setScheduledTransactions': {
            return action.payload;
        }
        case 'editScheduledTransaction': {
            return state.map(scheduledTransaction => {
                if (scheduledTransaction.id === parseInt(action.payload.id)) {
                    return action.payload;
                }

                return scheduledTransaction;
            });
        }
        case 'deleteScheduledTransaction': {
            return state.filter(scheduledTransaction => scheduledTransaction.id !== action.payload);
        }
        default:
            return state;
    }
};

export {
    scheduledTransactionsReducer
}