import React from 'react';
import {Link} from 'react-router-dom'
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import {Container, IconButton} from '@mui/material';
import {Add} from '@mui/icons-material';
import AccountCard from "./AccountCard";
import {useSelector} from "react-redux";

const AccountList = () => {
    const accounts = useSelector(state => state.accounts);

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Accounts</Typography>
                    <IconButton color='inherit' component={Link} to={'/accounts/new'}>
                        <Add/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                {
                    [...accounts]
                        .sort((a, b) => a.name.localeCompare(b.name))
                        .map(account =>
                            <AccountCard
                                key={account.id}
                                accountId={account.id}
                                accountName={account.name}
                                accountBalance={account.balance}
                            />
                        )
                }
            </Container>
        </>
    );
};

export default AccountList;
