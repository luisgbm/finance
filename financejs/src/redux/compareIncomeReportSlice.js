// Holds the user's "Compare Income Categories" report options so they survive navigation
// between the report page and the category-selection screen (and across app restarts via
// redux-persist).
//
// `selectedCategoryIds === null` means "All categories" (the default); an array is an
// explicit selection (which may be empty). Dates are stored as 'YYYY-MM-DD' strings to keep
// the store serializable. The selected categories and chart type are shared by both tabs;
// the period is per-tab because the Historical and Forecast tabs offer different period
// options.
const initialState = {
    activeTab: 0,
    selectedCategoryIds: null,
    chartType: 'Pie',
    period: 'last_6_months',
    customStart: null,
    customEnd: null,
    forecastPeriod: 'next_3_months',
    forecastCustomStart: null,
    forecastCustomEnd: null,
};

const compareIncomeReportReducer = (state = initialState, action) => {
    switch (action.type) {
        case 'setCompareIncomeTab': {
            return {...state, activeTab: action.payload};
        }
        case 'setCompareIncomeCategories': {
            return {...state, selectedCategoryIds: action.payload};
        }
        case 'setCompareIncomeChartType': {
            return {...state, chartType: action.payload};
        }
        case 'setCompareIncomePeriod': {
            return {...state, period: action.payload};
        }
        case 'setCompareIncomeCustomStart': {
            return {...state, customStart: action.payload};
        }
        case 'setCompareIncomeCustomEnd': {
            return {...state, customEnd: action.payload};
        }
        case 'setCompareIncomeForecastPeriod': {
            return {...state, forecastPeriod: action.payload};
        }
        case 'setCompareIncomeForecastCustomStart': {
            return {...state, forecastCustomStart: action.payload};
        }
        case 'setCompareIncomeForecastCustomEnd': {
            return {...state, forecastCustomEnd: action.payload};
        }
        default:
            return state;
    }
};

export {
    compareIncomeReportReducer
}
