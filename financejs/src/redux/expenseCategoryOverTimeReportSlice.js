// Holds the user's "Expense Category Over Time" report options so they survive navigation
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

const expenseCategoryOverTimeReportReducer = (state = initialState, action) => {
    switch (action.type) {
        case 'setExpenseCategoryOverTimeTab': {
            return {...state, activeTab: action.payload};
        }
        case 'setExpenseCategoryOverTimeCategories': {
            return {...state, selectedCategoryIds: action.payload};
        }
        case 'setExpenseCategoryOverTimeBreakdown': {
            return {...state, breakdown: action.payload};
        }
        case 'setExpenseCategoryOverTimePeriod': {
            return {...state, period: action.payload};
        }
        case 'setExpenseCategoryOverTimeCustomStart': {
            return {...state, customStart: action.payload};
        }
        case 'setExpenseCategoryOverTimeCustomEnd': {
            return {...state, customEnd: action.payload};
        }
        case 'setExpenseCategoryOverTimeForecastPeriod': {
            return {...state, forecastPeriod: action.payload};
        }
        case 'setExpenseCategoryOverTimeForecastCustomStart': {
            return {...state, forecastCustomStart: action.payload};
        }
        case 'setExpenseCategoryOverTimeForecastCustomEnd': {
            return {...state, forecastCustomEnd: action.payload};
        }
        default:
            return state;
    }
};

export {
    expenseCategoryOverTimeReportReducer
}
