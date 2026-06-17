import React from 'react';

import {Link, useNavigate, useParams} from 'react-router-dom'
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import AppBar from '@mui/material/AppBar';
import {Add} from '@mui/icons-material';
import ThumbUpIcon from '@mui/icons-material/ThumbUp';
import ThumbDownIcon from '@mui/icons-material/ThumbDown';
import {Container, IconButton, Tab, Tabs} from '@mui/material';
import CategoryCard from "./CategoryCard";
import {useSelector} from "react-redux";

const CategoryList = () => {
    const params = useParams();
    const navigate = useNavigate();

    const tabNameToValue = (tabName) => {
        let tabValue = 0;

        if (tabName) {
            if (tabName === 'expense') {
                tabValue = 0;
            } else {
                tabValue = 1;
            }
        }

        return tabValue;
    };

    const tabValueToName = (tabValue) => {
        return tabValue === 0 ? 'expense' : 'income';
    };

    const currentTab = tabNameToValue(params.type);

    const categories = useSelector(state => state.categories);

    const onChangeTab = (event, newValue) => {
        navigate(`/categories/${tabValueToName(newValue)}`);
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Categories</Typography>
                    <IconButton color='inherit' component={Link}
                                to={`/categories/new/${tabValueToName(currentTab)}`}>
                        <Add/>
                    </IconButton>
                </Toolbar>
                <Tabs value={currentTab} onChange={onChangeTab} centered>
                    <Tab icon={<ThumbDownIcon/>} label='Expenses'/>
                    <Tab icon={<ThumbUpIcon/>} label='Incomes'/>
                </Tabs>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                {
                    [...categories]
                        .filter(category => currentTab === 0 ?
                            category.categorytype === 'Expense' :
                            category.categorytype === 'Income')
                        .sort((a, b) => a.name.localeCompare(b.name))
                        .map(category =>
                            <CategoryCard
                                key={category.id}
                                categoryId={category.id}
                                categoryName={category.name}
                            />
                        )
                }
            </Container>
        </>
    );
};

export default CategoryList;
