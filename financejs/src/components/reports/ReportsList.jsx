import React from 'react';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import {Card, Container, List, ListItemButton, ListItemIcon, ListItemText} from '@mui/material';
import ShowChartIcon from '@mui/icons-material/ShowChart';
import PieChartIcon from '@mui/icons-material/PieChart';
import PieChartOutlineIcon from '@mui/icons-material/PieChartOutlined';
import InsertChartIcon from '@mui/icons-material/InsertChart';
import InsertChartOutlinedIcon from '@mui/icons-material/InsertChartOutlined';
import AssessmentIcon from '@mui/icons-material/Assessment';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import {Link} from 'react-router-dom';

const ReportsList = () => {
    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Reports</Typography>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <Card variant='outlined'>
                    <List disablePadding>
                        <ListItemButton component={Link} to='/reports/cashflow'>
                            <ListItemIcon>
                                <ShowChartIcon/>
                            </ListItemIcon>
                            <ListItemText primary='Cashflow'/>
                            <ChevronRightIcon color='action'/>
                        </ListItemButton>
                        <ListItemButton component={Link} to='/reports/compare-expenses'>
                            <ListItemIcon>
                                <PieChartIcon/>
                            </ListItemIcon>
                            <ListItemText primary='Compare Expense Categories'/>
                            <ChevronRightIcon color='action'/>
                        </ListItemButton>
                        <ListItemButton component={Link} to='/reports/compare-income'>
                            <ListItemIcon>
                                <PieChartOutlineIcon/>
                            </ListItemIcon>
                            <ListItemText primary='Compare Income Categories'/>
                            <ChevronRightIcon color='action'/>
                        </ListItemButton>
                        <ListItemButton component={Link} to='/reports/expense-category-over-time'>
                            <ListItemIcon>
                                <InsertChartIcon/>
                            </ListItemIcon>
                            <ListItemText primary='Expense Category Over Time'/>
                            <ChevronRightIcon color='action'/>
                        </ListItemButton>
                        <ListItemButton component={Link} to='/reports/income-category-over-time'>
                            <ListItemIcon>
                                <InsertChartOutlinedIcon/>
                            </ListItemIcon>
                            <ListItemText primary='Income Category Over Time'/>
                            <ChevronRightIcon color='action'/>
                        </ListItemButton>
                        <ListItemButton component={Link} to='/reports/statistics'>
                            <ListItemIcon>
                                <AssessmentIcon/>
                            </ListItemIcon>
                            <ListItemText primary='Statistics'/>
                            <ChevronRightIcon color='action'/>
                        </ListItemButton>
                    </List>
                </Card>
            </Container>
        </>
    );
};

export default ReportsList;
