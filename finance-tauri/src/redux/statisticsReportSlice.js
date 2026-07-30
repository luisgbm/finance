// Holds the user's Statistics-report options so they survive navigation between the Statistics
// page and the account-selection screen (and across app restarts via redux-persist).
//
// `selectedAccountIds === null` means "All accounts" (the default); an array is an explicit
// selection (which may be empty). Dates are stored as 'YYYY-MM-DD' strings to keep the store
// serializable. The period is per-tab because the Historical and Forecast tabs offer different
// period options.
const initialState = {
    activeTab: 0,
    selectedAccountIds: null,
    period: 'last_6_months',
    customStart: null,
    customEnd: null,
    forecastPeriod: 'next_3_months',
    forecastCustomStart: null,
    forecastCustomEnd: null,
};

const statisticsReportReducer = (state = initialState, action) => {
    switch (action.type) {
        case 'setStatisticsTab': {
            return {...state, activeTab: action.payload};
        }
        case 'setStatisticsAccounts': {
            return {...state, selectedAccountIds: action.payload};
        }
        case 'setStatisticsPeriod': {
            return {...state, period: action.payload};
        }
        case 'setStatisticsCustomStart': {
            return {...state, customStart: action.payload};
        }
        case 'setStatisticsCustomEnd': {
            return {...state, customEnd: action.payload};
        }
        case 'setStatisticsForecastPeriod': {
            return {...state, forecastPeriod: action.payload};
        }
        case 'setStatisticsForecastCustomStart': {
            return {...state, forecastCustomStart: action.payload};
        }
        case 'setStatisticsForecastCustomEnd': {
            return {...state, forecastCustomEnd: action.payload};
        }
        default:
            return state;
    }
};

export {
    statisticsReportReducer
}
