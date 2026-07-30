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
import {computeForecastStatistics, computeHistoricalStatistics} from '../../utils/statistics';

const Statistics = () => {
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const accounts = useSelector(state => state.accounts);
    const scheduledTransactions = useSelector(state => state.scheduledTransactions);
    const {
        activeTab,
        selectedAccountIds,
        period,
        customStart,
        customEnd,
        forecastPeriod,
        forecastCustomStart,
        forecastCustomEnd,
    } = useSelector(state => state.statisticsReport);

    const isForecast = activeTab === 1;
    const periodOptions = isForecast ? FORECAST_PERIOD_OPTIONS : PERIOD_OPTIONS;
    const currentPeriod = isForecast ? forecastPeriod : period;
    const currentCustomStart = isForecast ? forecastCustomStart : customStart;
    const currentCustomEnd = isForecast ? forecastCustomEnd : customEnd;
    const periodAction = isForecast ? 'setStatisticsForecastPeriod' : 'setStatisticsPeriod';
    const customStartAction = isForecast ? 'setStatisticsForecastCustomStart' : 'setStatisticsCustomStart';
    const customEndAction = isForecast ? 'setStatisticsForecastCustomEnd' : 'setStatisticsCustomEnd';
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
        let subtitle = `${modeLabel} · ${periodOpt ? periodOpt.label : ''}`;
        if (currentPeriod === 'custom') {
            subtitle += ` (${moment(currentCustomStart, 'YYYY-MM-DD').format('DD/MM/YYYY')}` +
                ` – ${moment(currentCustomEnd, 'YYYY-MM-DD').format('DD/MM/YYYY')})`;
        }
        return subtitle;
    };

    // Sum of the selected accounts' current balances (null = all accounts) — the "today" snapshot
    // shown as Current net worth and the anchor for the net-worth timeline.
    const currentNetWorth = () => {
        const selectedSet = selectedAccountIds === null ? null : new Set(selectedAccountIds);
        return accounts
            .filter(a => selectedSet === null || selectedSet.has(a.id))
            .reduce((sum, a) => sum + a.balance, 0);
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

        const netWorth = currentNetWorth();

        // Forecast is derived synchronously from the already-loaded scheduled transactions.
        if (isForecast) {
            const range = forecastPeriodToRange(currentPeriod, currentCustomStart, currentCustomEnd);
            const data = computeForecastStatistics(scheduledTransactions, range, selectedAccountIds, netWorth);
            navigate('/reports/statistics/result', {state: {...data, subtitle: buildSubtitle()}});
            return;
        }

        try {
            toggleLoadingModalOpen();

            const results = await Promise.all(
                ids.map(id => transactionService.getAllTransactionsForAccountId(id))
            );
            const transactions = results.flatMap(r => r.data);

            const range = periodToRange(currentPeriod, currentCustomStart, currentCustomEnd);
            const data = computeHistoricalStatistics(transactions, range, netWorth);

            toggleLoadingModalOpen();

            navigate('/reports/statistics/result', {state: {...data, subtitle: buildSubtitle()}});
        } catch (e) {
            toggleLoadingModalOpen();
            showMessageModal('Error', 'An error occurred while building the report, please try again.');
        }
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Statistics</Typography>
                    <IconButton color='inherit' onClick={onConfirm}>
                        <DoneIcon/>
                    </IconButton>
                </Toolbar>
                <Tabs
                    value={activeTab}
                    onChange={(e, v) => dispatch({type: 'setStatisticsTab', payload: v})}
                    centered
                >
                    <Tab label='Historical'/>
                    <Tab label='Forecast'/>
                </Tabs>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <Card variant='outlined' sx={{mb: 3}}>
                    <List disablePadding>
                        <ListItemButton component={Link} to='/reports/statistics/accounts'>
                            <ListItemText primary='Accounts' secondary={accountsSummary()}/>
                            <ChevronRightIcon color='action'/>
                        </ListItemButton>
                    </List>
                </Card>

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

export default Statistics;
