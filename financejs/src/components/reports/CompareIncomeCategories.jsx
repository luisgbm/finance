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

const CompareIncomeCategories = () => {
    const navigate = useNavigate();
    const dispatch = useDispatch();

    const categories = useSelector(state => state.categories);
    const selectedCategoryIds = useSelector(state => state.compareIncomeReport.selectedCategoryIds);

    // This report only ever compares income categories, so expense categories are not listed.
    const incomeCategories = [...categories]
        .filter(c => c.categorytype === 'Income')
        .sort((a, b) => a.name.localeCompare(b.name));
    const allIds = incomeCategories.map(c => c.id);

    // null means "All categories" — render that as every box checked.
    const selectedSet = new Set(selectedCategoryIds === null ? allIds : selectedCategoryIds);

    const isChecked = (id) => selectedSet.has(id);

    const toggle = (id) => {
        const next = new Set(selectedSet);
        if (next.has(id)) {
            next.delete(id);
        } else {
            next.add(id);
        }
        dispatch({type: 'setCompareIncomeCategories', payload: allIds.filter(x => next.has(x))});
    };

    const selectAll = () => {
        dispatch({type: 'setCompareIncomeCategories', payload: [...allIds]});
    };

    const deselectAll = () => {
        dispatch({type: 'setCompareIncomeCategories', payload: []});
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <IconButton color='inherit' edge='start' onClick={() => navigate('/reports/compare-income')}>
                        <ArrowBackIcon/>
                    </IconButton>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Select Categories</Typography>
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
                            incomeCategories.map(category =>
                                <ListItemButton key={category.id} onClick={() => toggle(category.id)}>
                                    <ListItemIcon>
                                        <Checkbox
                                            edge='start'
                                            checked={isChecked(category.id)}
                                            tabIndex={-1}
                                            disableRipple
                                        />
                                    </ListItemIcon>
                                    <ListItemText primary={category.name}/>
                                </ListItemButton>
                            )
                        }
                    </List>
                </Card>
            </Container>
        </>
    );
};

export default CompareIncomeCategories;
