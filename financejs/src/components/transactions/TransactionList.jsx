import React, {useContext, useEffect} from 'react';

import {Link, useNavigate, useParams} from 'react-router-dom'
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import AppBar from '@mui/material/AppBar';
import {Add} from '@mui/icons-material';
import {Chip, Container, IconButton} from '@mui/material';
import {transactionService} from "../../api/transaction.service";
import TransferCard from "./TransferCard";
import TransactionCard from "./TransactionCard";
import {moneyFormat} from "../../utils/utils";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useSelector} from "react-redux";

const TransactionList = () => {
    const params = useParams();
    const navigate = useNavigate();

    const accountId = parseInt(params.accountId);

    const accounts = useSelector(state => state.accounts);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const [transactions, setTransactions] = React.useState([]);
    const [accountName, setAccountName] = React.useState('');
    const [accountBalance, setAccountBalance] = React.useState(0);

    useEffect(() => {
        (async function loadTransactionData() {
            try {
                toggleLoadingModalOpen();
                const transactions = await transactionService.getAllTransactionsForAccountId(accountId);

                const account = accounts.find(a => a.id === parseInt(accountId));

                setTransactions(transactions.data);
                setAccountName(account.name);
                setAccountBalance(moneyFormat(account.balance));
                toggleLoadingModalOpen();
            } catch (e) {
                if (e.response && e.response.status === 401) {
                    navigate('/')
                }

                toggleLoadingModalOpen();
                showMessageModal('Error', 'An error occurred while processing your request, please try again.');
            }
        })()
    }, []); // eslint-disable-line react-hooks/exhaustive-deps

    const getCard = (transaction) => {
        if (transaction.category_type === 'TransferIncome' || transaction.category_type === 'TransferExpense') {
            return (
                <TransferCard
                    key={transaction.id}
                    transaction={transaction}
                    fromAccountId={accountId}
                />
            );
        } else {
            return (
                <TransactionCard
                    key={transaction.id}
                    transaction={transaction}
                />
            );
        }
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>{accountName} <Chip
                        label={accountBalance}/></Typography>
                    <IconButton
                        color='inherit'
                        component={Link}
                        to={`/transactions/account/${accountId}/new/transaction`}
                    >
                        <Add/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                {
                    transactions.map(transaction =>
                        getCard(transaction)
                    )
                }
            </Container>
        </>
    );
};

export default TransactionList;
