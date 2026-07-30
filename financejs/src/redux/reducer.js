import {combineReducers} from '@reduxjs/toolkit'
import {accountsReducer} from './accountsSlice';
import {categoriesReducer} from './categoriesSlice';
import {scheduledTransactionsReducer} from './scheduledTransactionsSlice';
import {cashflowReportReducer} from './cashflowReportSlice';
import {compareExpensesReportReducer} from './compareExpensesReportSlice';
import {compareIncomeReportReducer} from './compareIncomeReportSlice';

const rootReducer = combineReducers({
    accounts: accountsReducer,
    categories: categoriesReducer,
    scheduledTransactions: scheduledTransactionsReducer,
    cashflowReport: cashflowReportReducer,
    compareExpensesReport: compareExpensesReportReducer,
    compareIncomeReport: compareIncomeReportReducer
})

export default rootReducer