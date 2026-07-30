// Holds the user's "Income Category Over Time" report options so they survive navigation
// between the report page and the category-selection screen (and across app restarts via
// redux-persist).
//
// `selectedCategoryIds === null` means "All categories" (the default); an array is an
// explicit selection (which may be empty). Dates are stored as 'YYYY-MM-DD' strings to keep
// the store serializable. The selected categories and breakdown are shared by both tabs; the
// period is per-tab because the Historical and Forecast tabs offer different period options.
const initialState = {
    activeTab: 0,
    selectedCategoryIds: null,
    breakdown: 'Months',
    period: 'last_6_months',
    customStart: null,
    customEnd: null,
    forecastPeriod: 'next_3_months',
    forecastCustomStart: null,
    forecastCustomEnd: null,
};

const incomeCategoryOverTimeReportReducer = (state = initialState, action) => {
    switch (action.type) {
        case 'setIncomeCategoryOverTimeTab': {
            return {...state, activeTab: action.payload};
        }
        case 'setIncomeCategoryOverTimeCategories': {
            return {...state, selectedCategoryIds: action.payload};
        }
        case 'setIncomeCategoryOverTimeBreakdown': {
            return {...state, breakdown: action.payload};
        }
        case 'setIncomeCategoryOverTimePeriod': {
            return {...state, period: action.payload};
        }
        case 'setIncomeCategoryOverTimeCustomStart': {
            return {...state, customStart: action.payload};
        }
        case 'setIncomeCategoryOverTimeCustomEnd': {
            return {...state, customEnd: action.payload};
        }
        case 'setIncomeCategoryOverTimeForecastPeriod': {
            return {...state, forecastPeriod: action.payload};
        }
        case 'setIncomeCategoryOverTimeForecastCustomStart': {
            return {...state, forecastCustomStart: action.payload};
        }
        case 'setIncomeCategoryOverTimeForecastCustomEnd': {
            return {...state, forecastCustomEnd: action.payload};
        }
        default:
            return state;
    }
};

export {
    incomeCategoryOverTimeReportReducer
}
