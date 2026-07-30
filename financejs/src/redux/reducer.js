import {combineReducers} from '@reduxjs/toolkit'
import {accountsReducer} from './accountsSlice';
import {categoriesReducer} from './categoriesSlice';
import {scheduledTransactionsReducer} from './scheduledTransactionsSlice';
import {cashflowReportReducer} from './cashflowReportSlice';

const rootReducer = combineReducers({
    accounts: accountsReducer,
    categories: categoriesReducer,
    scheduledTransactions: scheduledTransactionsReducer,
    cashflowReport: cashflowReportReducer
})

export default rootReducer