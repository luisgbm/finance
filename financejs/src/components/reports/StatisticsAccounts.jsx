import React from 'react';
import {useNavigate} from 'react-router-dom';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import {
    Button,
    Card,
    Checkbox,
    Container,
    IconButton,
    List,
    ListItemButton,
    ListItemIcon,
    ListItemText,
    Stack,
} from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import {useDispatch, useSelector} from 'react-redux';

const StatisticsAccounts = () => {
    const navigate = useNavigate();
    const dispatch = useDispatch();

    const accounts = useSelector(state => state.accounts);
    const selectedAccountIds = useSelector(state => state.statisticsReport.selectedAccountIds);

    const sortedAccounts = [...accounts].sort((a, b) => a.name.localeCompare(b.name));
    const allIds = sortedAccounts.map(a => a.id);

    // null means "All accounts" — render that as every box checked.
    const selectedSet = new Set(selectedAccountIds === null ? allIds : selectedAccountIds);

    const isChecked = (id) => selectedSet.has(id);

    const toggle = (id) => {
        const next = new Set(selectedSet);
        if (next.has(id)) {
            next.delete(id);
        } else {
            next.add(id);
        }
        dispatch({type: 'setStatisticsAccounts', payload: allIds.filter(x => next.has(x))});
    };

    const selectAll = () => {
        dispatch({type: 'setStatisticsAccounts', payload: [...allIds]});
    };

    const deselectAll = () => {
        dispatch({type: 'setStatisticsAccounts', payload: []});
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <IconButton color='inherit' edge='start' onClick={() => navigate('/reports/statistics')}>
                        <ArrowBackIcon/>
                    </IconButton>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Select Accounts</Typography>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <Stack direction='row' spacing={2} sx={{mb: 2}}>
                    <Button variant='outlined' fullWidth onClick={selectAll}>Select all</Button>
                    <Button variant='outlined' fullWidth onClick={deselectAll}>Deselect all</Button>
                </Stack>
                <Card variant='outlined'>
                    <List disablePadding>
                        {
                            sortedAccounts.map(account =>
                                <ListItemButton key={account.id} onClick={() => toggle(account.id)}>
                                    <ListItemIcon>
                                        <Checkbox
                                            edge='start'
                                            checked={isChecked(account.id)}
                                            tabIndex={-1}
                                            disableRipple
                                        />
                                    </ListItemIcon>
                                    <ListItemText primary={account.name}/>
                                </ListItemButton>
                            )
                        }
                    </List>
                </Card>
            </Container>
        </>
    );
};

export default StatisticsAccounts;
