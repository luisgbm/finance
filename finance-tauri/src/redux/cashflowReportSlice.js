// Holds the user's Cashflow-report options so they survive navigation between the Cashflow
// page and the account-selection screen (and across app restarts via redux-persist).
//
// `selectedAccountIds === null` means "All accounts" (the default); an array is an explicit
// selection (which may be empty). Dates are stored as 'YYYY-MM-DD' strings to keep the store
// serializable. Accounts and breakdown are shared by both tabs; the period is per-tab because
// the Historical and Forecast tabs offer different period options.
const initialState = {
    activeTab: 0,
    selectedAccountIds: null,
    breakdown: 'Months',
    period: 'last_6_months',
    customStart: null,
    customEnd: null,
    forecastPeriod: 'next_3_months',
    forecastCustomStart: null,
    forecastCustomEnd: null,
};

const cashflowReportReducer = (state = initialState, action) => {
    switch (action.type) {
        case 'setCashflowTab': {
            return {...state, activeTab: action.payload};
        }
        case 'setCashflowAccounts': {
            return {...state, selectedAccountIds: action.payload};
        }
        case 'setCashflowBreakdown': {
            return {...state, breakdown: action.payload};
        }
        case 'setCashflowPeriod': {
            return {...state, period: action.payload};
        }
        case 'setCashflowCustomStart': {
            return {...state, customStart: action.payload};
        }
        case 'setCashflowCustomEnd': {
            return {...state, customEnd: action.payload};
        }
        case 'setCashflowForecastPeriod': {
            return {...state, forecastPeriod: action.payload};
        }
        case 'setCashflowForecastCustomStart': {
            return {...state, forecastCustomStart: action.payload};
        }
        case 'setCashflowForecastCustomEnd': {
            return {...state, forecastCustomEnd: action.payload};
        }
        default:
            return state;
    }
};

export {
    cashflowReportReducer
}
