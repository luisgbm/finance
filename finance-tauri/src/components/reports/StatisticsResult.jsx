import React from 'react';
import {Navigate, useLocation, useNavigate} from 'react-router-dom';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import {Card, CardContent, Container, Divider, IconButton, Stack} from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import moment from 'moment';
import {moneyFormat} from '../../utils/utils';

const EXPENSE_COLOR = 'red';
const INCOME_COLOR = 'green';

// A label on the left and a right-aligned value (optionally coloured).
const StatRow = ({label, value, color}) => (
    <Stack direction='row' justifyContent='space-between' alignItems='baseline' sx={{py: 0.5}}>
        <Typography variant='body2' color='text.secondary'>{label}</Typography>
        <Typography variant='body1' sx={{fontWeight: 500, color}}>{value}</Typography>
    </Stack>
);

const SectionCard = ({title, children}) => (
    <Card variant='outlined' sx={{mb: 3}}>
        <CardContent>
            <Typography variant='subtitle1' sx={{fontWeight: 600, mb: 1}}>{title}</Typography>
            {children}
        </CardContent>
    </Card>
);

// Renders the top-N category list, or a muted placeholder when there is nothing to show.
const CategoryList = ({categories, color}) => {
    if (!categories || categories.length === 0) {
        return <Typography variant='body2' color='text.secondary'>No data for this period.</Typography>;
    }
    return (
        <>
            {categories.map((c, i) =>
                <StatRow key={i} label={c.name} value={moneyFormat(c.value)} color={color}/>
            )}
        </>
    );
};

const formatDay = (s) => (s ? moment(s, 'YYYY-MM-DD').format('DD/MM/YYYY') : '—');
const netWorthColor = (v) => (v >= 0 ? INCOME_COLOR : EXPENSE_COLOR);

const StatisticsResult = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const state = location.state;

    // The stats are handed over via router state when the user confirms on the Statistics page.
    // If this route is reached without them (e.g. a page reload), go back to the options.
    if (!state || state.kind !== 'statistics') {
        return <Navigate to='/reports/statistics' replace/>;
    }

    const {expenses, incomes, counts} = state;

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <IconButton color='inherit' edge='start' onClick={() => navigate('/reports/statistics')}>
                        <ArrowBackIcon/>
                    </IconButton>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Statistics</Typography>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                {
                    state.subtitle &&
                    <Typography variant='body2' color='text.secondary' sx={{mb: 2}}>{state.subtitle}</Typography>
                }

                <SectionCard title='Expenses'>
                    <StatRow label='Daily average' value={moneyFormat(expenses.daily)} color={EXPENSE_COLOR}/>
                    <StatRow label='Monthly average' value={moneyFormat(expenses.monthly)} color={EXPENSE_COLOR}/>
                    <StatRow label='Yearly average' value={moneyFormat(expenses.yearly)} color={EXPENSE_COLOR}/>
                    <Divider sx={{my: 1}}/>
                    <StatRow label='Total for period' value={moneyFormat(expenses.total)} color={EXPENSE_COLOR}/>
                </SectionCard>

                <SectionCard title='Incomes'>
                    <StatRow label='Daily average' value={moneyFormat(incomes.daily)} color={INCOME_COLOR}/>
                    <StatRow label='Monthly average' value={moneyFormat(incomes.monthly)} color={INCOME_COLOR}/>
                    <StatRow label='Yearly average' value={moneyFormat(incomes.yearly)} color={INCOME_COLOR}/>
                    <Divider sx={{my: 1}}/>
                    <StatRow label='Total for period' value={moneyFormat(incomes.total)} color={INCOME_COLOR}/>
                </SectionCard>

                <SectionCard title='Top 5 Expense categories'>
                    <CategoryList categories={state.topExpenseCategories} color={EXPENSE_COLOR}/>
                </SectionCard>

                <SectionCard title='Top 5 Income categories'>
                    <CategoryList categories={state.topIncomeCategories} color={INCOME_COLOR}/>
                </SectionCard>

                <SectionCard title='Net worth'>
                    <StatRow
                        label='Current'
                        value={moneyFormat(state.currentNetWorth)}
                        color={netWorthColor(state.currentNetWorth)}
                    />
                    <StatRow
                        label='Highest'
                        value={moneyFormat(state.highestNetWorth)}
                        color={netWorthColor(state.highestNetWorth)}
                    />
                    <StatRow
                        label='Lowest'
                        value={moneyFormat(state.lowestNetWorth)}
                        color={netWorthColor(state.lowestNetWorth)}
                    />
                </SectionCard>

                <SectionCard title='Transactions'>
                    <StatRow label='Expenses' value={counts.expenses}/>
                    <StatRow label='Incomes' value={counts.incomes}/>
                    <StatRow label='Transfers' value={counts.transfers}/>
                </SectionCard>

                <SectionCard title='Period'>
                    <StatRow label='Start date' value={formatDay(state.periodStart)}/>
                    <StatRow label='End date' value={formatDay(state.periodEnd)}/>
                </SectionCard>
            </Container>
        </>
    );
};

export default StatisticsResult;
