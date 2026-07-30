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
import {useSelector} from 'react-redux';
import {useDispatch} from 'react-redux';
import LoadingModalContext from '../../context/LoadingModalContext';
import MessageModalContext from '../../context/MessageModalContext';
import {transactionService} from '../../api/transaction.service';
import {
    BREAKDOWN_OPTIONS,
    FORECAST_PERIOD_OPTIONS,
    PERIOD_OPTIONS,
    computeCashflow,
    computeForecast,
    forecastPeriodToRange,
    periodToRange,
} from '../../utils/cashflow';

const Cashflow = () => {
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const accounts = useSelector(state => state.accounts);
    const scheduledTransactions = useSelector(state => state.scheduledTransactions);
    const {
        activeTab,
        selectedAccountIds,
        breakdown,
        period,
        customStart,
        customEnd,
        forecastPeriod,
        forecastCustomStart,
        forecastCustomEnd,
    } = useSelector(state => state.cashflowReport);

    const isForecast = activeTab === 1;
    const periodOptions = isForecast ? FORECAST_PERIOD_OPTIONS : PERIOD_OPTIONS;
    const currentPeriod = isForecast ? forecastPeriod : period;
    const currentCustomStart = isForecast ? forecastCustomStart : customStart;
    const currentCustomEnd = isForecast ? forecastCustomEnd : customEnd;
    const periodAction = isForecast ? 'setCashflowForecastPeriod' : 'setCashflowPeriod';
    const customStartAction = isForecast ? 'setCashflowForecastCustomStart' : 'setCashflowCustomStart';
    const customEndAction = isForecast ? 'setCashflowForecastCustomEnd' : 'setCashflowCustomEnd';
    // Historical looks at the past, Forecast at the future — bound the custom date pickers accordingly.
    const dateBounds = isForecast ? {minDate: moment()} : {maxDate: moment()};

    const accountsSummary = () => {
        const total = accounts.length;
        if (selectedAccountIds === null || selectedAccountIds.length === total) {
            return 'All accounts';
        }
        if (selectedAccountIds.length === 0) {
            return 'No accounts selected';
        }
        return `${selectedAccountIds.length} of ${total} accounts`;
    };

    const buildSubtitle = () => {
        const periodOpt = periodOptions.find(o => o.value === currentPeriod);
        const modeLabel = isForecast ? 'Forecast' : 'Historical';
        let subtitle = `${modeLabel} · ${breakdown} · ${periodOpt ? periodOpt.label : ''}`;
        if (currentPeriod === 'custom') {
            subtitle += ` (${moment(currentCustomStart, 'YYYY-MM-DD').format('DD/MM/YYYY')}` +
                ` – ${moment(currentCustomEnd, 'YYYY-MM-DD').format('DD/MM/YYYY')})`;
        }
        return subtitle;
    };

    const onConfirm = async () => {
        const allIds = accounts.map(a => a.id);
        const ids = selectedAccountIds === null ? allIds : selectedAccountIds;

        if (ids.length === 0) {
            showMessageModal('Error', 'Please select at least one account for this report.');
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
            const data = computeForecast(scheduledTransactions, breakdown, range, selectedAccountIds);
            navigate('/reports/cashflow/chart', {state: {...data, subtitle: buildSubtitle()}});
            return;
        }

        try {
            toggleLoadingModalOpen();

            const results = await Promise.all(
                ids.map(id => transactionService.getAllTransactionsForAccountId(id))
            );
            const transactions = results.flatMap(r => r.data);

            const range = periodToRange(currentPeriod, currentCustomStart, currentCustomEnd);
            const data = computeCashflow(transactions, breakdown, range);

            toggleLoadingModalOpen();

            navigate('/reports/cashflow/chart', {state: {...data, subtitle: buildSubtitle()}});
        } catch (e) {
            toggleLoadingModalOpen();
            showMessageModal('Error', 'An error occurred while building the report, please try again.');
        }
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Cashflow</Typography>
                    <IconButton color='inherit' onClick={onConfirm}>
                        <DoneIcon/>
                    </IconButton>
                </Toolbar>
                <Tabs
                    value={activeTab}
                    onChange={(e, v) => dispatch({type: 'setCashflowTab', payload: v})}
                    centered
                >
                    <Tab label='Historical'/>
                    <Tab label='Forecast'/>
                </Tabs>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <Card variant='outlined' sx={{mb: 3}}>
                    <List disablePadding>
                        <ListItemButton component={Link} to='/reports/cashflow/accounts'>
                            <ListItemText primary='Accounts' secondary={accountsSummary()}/>
                            <ChevronRightIcon color='action'/>
                        </ListItemButton>
                    </List>
                </Card>

                <FormControl fullWidth variant='outlined' sx={{mb: 3}}>
                    <InputLabel id='breakdown-label'>Breakdown</InputLabel>
                    <Select
                        labelId='breakdown-label'
                        id='breakdown'
                        label='Breakdown'
                        value={breakdown}
                        onChange={e => dispatch({type: 'setCashflowBreakdown', payload: e.target.value})}
                    >
                        {
                            BREAKDOWN_OPTIONS.map(o =>
                                <MenuItem value={o.value} key={o.value}>{o.label}</MenuItem>
                            )
                        }
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

export default Cashflow;
