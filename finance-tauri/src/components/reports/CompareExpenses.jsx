import React, {useContext} from 'react';
import {Link, useNavigate} from 'react-router-dom';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import {
    Card,
    Container,
    FormControl,
    IconButton,
    InputLabel,
    List,
    ListItemButton,
    ListItemText,
    MenuItem,
    Select,
    Stack,
    Tab,
    Tabs,
} from '@mui/material';
import DoneIcon from '@mui/icons-material/Done';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import {DatePicker} from '@mui/x-date-pickers/DatePicker';
import moment from 'moment';
import {useDispatch, useSelector} from 'react-redux';
import LoadingModalContext from '../../context/LoadingModalContext';
import MessageModalContext from '../../context/MessageModalContext';
import {transactionService} from '../../api/transaction.service';
import {
    FORECAST_PERIOD_OPTIONS,
    PERIOD_OPTIONS,
    forecastPeriodToRange,
    periodToRange,
} from '../../utils/cashflow';
import {
    computeExpensesByCategory,
    computeForecastExpensesByCategory,
} from '../../utils/compareExpenses';

const CompareExpenses = () => {
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const accounts = useSelector(state => state.accounts);
    const categories = useSelector(state => state.categories);
    const scheduledTransactions = useSelector(state => state.scheduledTransactions);
    const {
        activeTab,
        selectedCategoryIds,
        chartType,
        period,
        customStart,
        customEnd,
        forecastPeriod,
        forecastCustomStart,
        forecastCustomEnd,
    } = useSelector(state => state.compareExpensesReport);

    const expenseCategories = categories.filter(c => c.categorytype === 'Expense');
    const allExpenseIds = expenseCategories.map(c => c.id);

    const isForecast = activeTab === 1;
    const periodOptions = isForecast ? FORECAST_PERIOD_OPTIONS : PERIOD_OPTIONS;
    const currentPeriod = isForecast ? forecastPeriod : period;
    const currentCustomStart = isForecast ? forecastCustomStart : customStart;
    const currentCustomEnd = isForecast ? forecastCustomEnd : customEnd;
    const periodAction = isForecast ? 'setCompareExpensesForecastPeriod' : 'setCompareExpensesPeriod';
    const customStartAction = isForecast ? 'setCompareExpensesForecastCustomStart' : 'setCompareExpensesCustomStart';
    const customEndAction = isForecast ? 'setCompareExpensesForecastCustomEnd' : 'setCompareExpensesCustomEnd';
    // Historical looks at the past, Forecast at the future — bound the custom date pickers accordingly.
    const dateBounds = isForecast ? {minDate: moment()} : {maxDate: moment()};

    const categoriesSummary = () => {
        const total = expenseCategories.length;
        if (selectedCategoryIds === null || selectedCategoryIds.length === total) {
            return 'All categories';
        }
        if (selectedCategoryIds.length === 0) {
            return 'No categories selected';
        }
        return `${selectedCategoryIds.length} of ${total} categories`;
    };

    const buildSubtitle = () => {
        const periodOpt = periodOptions.find(o => o.value === currentPeriod);
        const modeLabel = isForecast ? 'Forecast' : 'Historical';
        const chartLabel = chartType === 'Bar' ? 'Bar' : 'Pie';
        let subtitle = `${modeLabel} · ${chartLabel} · ${periodOpt ? periodOpt.label : ''}`;
        if (currentPeriod === 'custom') {
            subtitle += ` (${moment(currentCustomStart, 'YYYY-MM-DD').format('DD/MM/YYYY')}` +
                ` – ${moment(currentCustomEnd, 'YYYY-MM-DD').format('DD/MM/YYYY')})`;
        }
        return subtitle;
    };

    const onConfirm = async () => {
        const ids = selectedCategoryIds === null ? allExpenseIds : selectedCategoryIds;

        if (ids.length === 0) {
            showMessageModal('Error', 'Please select at least one category for this report.');
            return;
        }

        if (currentPeriod === 'custom') {
            if (!currentCustomStart || !currentCustomEnd) {
                showMessageModal('Error', 'Please select both a start and an end date for the custom period.');
                return;
            }
            if (moment(currentCustomStart, 'YYYY-MM-DD').isAfter(moment(currentCustomEnd, 'YYYY-MM-DD'))) {
                showMessageModal('Error', 'The start date must be on or before the end date.');
                return;
            }
        }

        // Forecast is derived synchronously from the already-loaded scheduled transactions.
        if (isForecast) {
            const range = forecastPeriodToRange(currentPeriod, currentCustomStart, currentCustomEnd);
            const data = computeForecastExpensesByCategory(scheduledTransactions, range, selectedCategoryIds);
            navigate('/reports/compare-expenses/chart', {state: {...data, chartType, subtitle: buildSubtitle()}});
            return;
        }

        try {
            toggleLoadingModalOpen();

            // This report groups by category across every account, so pull all accounts.
            const results = await Promise.all(
                accounts.map(a => transactionService.getAllTransactionsForAccountId(a.id))
            );
            const transactions = results.flatMap(r => r.data);

            const range = periodToRange(currentPeriod, currentCustomStart, currentCustomEnd);
            const data = computeExpensesByCategory(transactions, range, selectedCategoryIds);

            toggleLoadingModalOpen();

            navigate('/reports/compare-expenses/chart', {state: {...data, chartType, subtitle: buildSubtitle()}});
        } catch (e) {
            toggleLoadingModalOpen();
            showMessageModal('Error', 'An error occurred while building the report, please try again.');
        }
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Compare Expense Categories</Typography>
                    <IconButton color='inherit' onClick={onConfirm}>
                        <DoneIcon/>
                    </IconButton>
                </Toolbar>
                <Tabs
                    value={activeTab}
                    onChange={(e, v) => dispatch({type: 'setCompareExpensesTab', payload: v})}
                    centered
                >
                    <Tab label='Historical'/>
                    <Tab label='Forecast'/>
                </Tabs>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <Card variant='outlined' sx={{mb: 3}}>
                    <List disablePadding>
                        <ListItemButton component={Link} to='/reports/compare-expenses/categories'>
                            <ListItemText primary='Categories' secondary={categoriesSummary()}/>
                            <ChevronRightIcon color='action'/>
                        </ListItemButton>
                    </List>
                </Card>

                <FormControl fullWidth variant='outlined' sx={{mb: 3}}>
                    <InputLabel id='chart-type-label'>Chart type</InputLabel>
                    <Select
                        labelId='chart-type-label'
                        id='chart-type'
                        label='Chart type'
                        value={chartType}
                        onChange={e => dispatch({type: 'setCompareExpensesChartType', payload: e.target.value})}
                    >
                        <MenuItem value='Pie'>Pie</MenuItem>
                        <MenuItem value='Bar'>Bar</MenuItem>
                    </Select>
                </FormControl>

                <FormControl fullWidth variant='outlined' sx={{mb: 3}}>
                    <InputLabel id='period-label'>Period</InputLabel>
                    <Select
                        labelId='period-label'
                        id='period'
                        label='Period'
                        value={currentPeriod}
                        onChange={e => dispatch({type: periodAction, payload: e.target.value})}
                    >
                        {
                            periodOptions.map(o =>
                                <MenuItem value={o.value} key={o.value}>{o.label}</MenuItem>
                            )
                        }
                    </Select>
                </FormControl>

                {
                    currentPeriod === 'custom' &&
                    <Stack direction='row' spacing={2} sx={{mb: 3}}>
                        <DatePicker
                            label='Start date'
                            value={currentCustomStart ? moment(currentCustomStart, 'YYYY-MM-DD') : null}
                            onChange={v => dispatch({
                                type: customStartAction,
                                payload: v && v.isValid() ? v.format('YYYY-MM-DD') : null
                            })}
                            format='DD/MM/YYYY'
                            {...dateBounds}
                            slotProps={{textField: {fullWidth: true}}}
                        />
                        <DatePicker
                            label='End date'
                            value={currentCustomEnd ? moment(currentCustomEnd, 'YYYY-MM-DD') : null}
                            onChange={v => dispatch({
                                type: customEndAction,
                                payload: v && v.isValid() ? v.format('YYYY-MM-DD') : null
                            })}
                            format='DD/MM/YYYY'
                            {...dateBounds}
                            slotProps={{textField: {fullWidth: true}}}
                        />
                    </Stack>
                }
            </Container>
        </>
    );
};

export default CompareExpenses;
