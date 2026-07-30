import React from 'react';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import {Card, Container, List, ListItemButton, ListItemIcon, ListItemText} from '@mui/material';
import ShowChartIcon from '@mui/icons-material/ShowChart';
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
                    </List>
                </Card>
            </Container>
        </>
    );
};

export default ReportsList;
