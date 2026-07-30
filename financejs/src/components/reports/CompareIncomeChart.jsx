import React from 'react';
import {Box, Typography} from '@mui/material';
import {PieChart} from '@mui/x-charts/PieChart';
import {BarChart} from '@mui/x-charts/BarChart';
import {moneyFormat} from '../../utils/utils';

// A fixed categorical palette so a category keeps a stable, distinct colour whether it is
// rendered as a pie slice or a bar. Colours repeat only if there are more categories than
// entries here.
const PALETTE = [
    '#3f51b5', '#f44336', '#4caf50', '#ff9800', '#9c27b0',
    '#00bcd4', '#e91e63', '#8bc34a', '#ffc107', '#795548',
    '#607d8b', '#03a9f4', '#cddc39', '#ff5722', '#673ab7',
    '#009688', '#ffeb3b', '#9e9e9e', '#2196f3', '#f06292',
];

// Compact currency label for the Bar chart Y-axis ticks (values are integer cents).
const compactMoney = (cents) => {
    const dollars = (cents || 0) / 100;
    return '$' + dollars.toLocaleString('en-US', {notation: 'compact', maximumFractionDigits: 1});
};

const CompareIncomeChart = ({labels, values, chartType}) => {
    if (!labels || labels.length === 0) {
        return (
            <Box sx={{mt: 3, textAlign: 'center'}}>
                <Typography color='text.secondary'>
                    No incomes found for the selected options.
                </Typography>
            </Box>
        );
    }

    const colors = labels.map((_, i) => PALETTE[i % PALETTE.length]);

    if (chartType === 'Bar') {
        return (
            <Box sx={{mt: 3, width: '100%'}}>
                <BarChart
                    height={360}
                    hideLegend
                    xAxis={[{
                        scaleType: 'band',
                        data: labels,
                        colorMap: {type: 'ordinal', values: labels, colors},
                    }]}
                    yAxis={[{valueFormatter: compactMoney, width: 60}]}
                    series={[{
                        data: values,
                        label: 'Incomes',
                        valueFormatter: (v) => (v == null ? '' : moneyFormat(v)),
                    }]}
                    margin={{top: 20, right: 10, bottom: 20, left: 10}}
                />
            </Box>
        );
    }

    // Pie (default).
    const pieData = labels.map((label, i) => ({
        id: i,
        value: values[i],
        label,
        color: colors[i],
    }));

    return (
        <Box sx={{mt: 3, width: '100%'}}>
            <PieChart
                height={360}
                series={[{
                    data: pieData,
                    valueFormatter: (item) => moneyFormat(item.value),
                    highlightScope: {faded: 'global', highlighted: 'item'},
                }]}
                margin={{top: 20, right: 10, bottom: 20, left: 10}}
            />
        </Box>
    );
};

export default CompareIncomeChart;
