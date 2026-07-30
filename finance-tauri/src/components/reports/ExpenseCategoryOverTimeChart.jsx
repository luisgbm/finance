import React from 'react';
import {Box, Typography} from '@mui/material';
import {BarChart} from '@mui/x-charts/BarChart';
import {moneyFormat} from '../../utils/utils';

// Compact currency label for the Y axis ticks (values are integer cents).
const compactMoney = (cents) => {
    const dollars = (cents || 0) / 100;
    return '$' + dollars.toLocaleString('en-US', {notation: 'compact', maximumFractionDigits: 1});
};

const ExpenseCategoryOverTimeChart = ({labels, values}) => {
    if (!labels || labels.length === 0) {
        return (
            <Box sx={{mt: 3, textAlign: 'center'}}>
                <Typography color='text.secondary'>
                    No expenses found for the selected options.
                </Typography>
            </Box>
        );
    }

    return (
        <Box sx={{mt: 3, width: '100%'}}>
            <BarChart
                height={360}
                hideLegend
                xAxis={[{scaleType: 'band', data: labels}]}
                yAxis={[{valueFormatter: compactMoney, width: 60}]}
                series={[{
                    data: values,
                    label: 'Expenses',
                    color: '#f44336',
                    valueFormatter: (v) => (v == null ? '' : moneyFormat(v)),
                }]}
                margin={{top: 20, right: 10, bottom: 20, left: 10}}
            />
        </Box>
    );
};

export default ExpenseCategoryOverTimeChart;
