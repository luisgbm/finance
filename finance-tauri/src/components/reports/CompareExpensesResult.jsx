import React from 'react';
import {Navigate, useLocation, useNavigate} from 'react-router-dom';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import {Container, IconButton} from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import CompareExpensesChart from './CompareExpensesChart';

const CompareExpensesResult = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const state = location.state;

    // The chart data is handed over via router state when the user confirms on the report
    // page. If this route is reached without it (e.g. a page reload), go back to the options.
    if (!state || !state.labels) {
        return <Navigate to='/reports/compare-expenses' replace/>;
    }

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <IconButton color='inherit' edge='start' onClick={() => navigate('/reports/compare-expenses')}>
                        <ArrowBackIcon/>
                    </IconButton>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Compare Expense Categories</Typography>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                {
                    state.subtitle &&
                    <Typography variant='body2' color='text.secondary'>{state.subtitle}</Typography>
                }
                <CompareExpensesChart
                    labels={state.labels}
                    values={state.values}
                    chartType={state.chartType}
                />
            </Container>
        </>
    );
};

export default CompareExpensesResult;
