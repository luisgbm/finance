import React, {useContext} from 'react';

import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import AppBar from '@mui/material/AppBar';
import {Container, IconButton, Tab, Tabs} from '@mui/material';
import DoneIcon from '@mui/icons-material/Done';
import moment from 'moment';
import {transactionService} from "../../api/transaction.service";
import AutorenewIcon from '@mui/icons-material/Autorenew';
import AttachMoneyIcon from '@mui/icons-material/AttachMoney';
import {useFormik} from "formik";
import * as yup from "yup";
import TransactionForm from "./TransactionForm";
import TransferForm from "./TransferForm";
import {transferService} from "../../api/transfer.service";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useDispatch} from "react-redux";
import {accountService} from "../../api/account.service";
import currency from "currency.js";
import {useNavigate, useParams} from "react-router-dom";

const NewTransaction = () => {
    const params = useParams();
    const navigate = useNavigate();

    const tabNameToValue = (tabName) => {
        let tabValue = 0;

        if (tabName) {
            if (tabName === 'transaction') {
                tabValue = 0;
            } else {
                tabValue = 1;
            }
        }

        return tabValue;
    };

    const tabValueToName = (tabValue) => {
        return tabValue === 0 ? 'transaction' : 'transfer';
    };

    const currentTab = tabNameToValue(params.type);
    const accountId = parseInt(params.accountId);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const dispatch = useDispatch();

    const formikTransaction = useFormik({
        initialValues: {
            value: '',
            description: '',
            accountId: accountId,
            categoryType: '',
            categoryId: '',
            date: moment()

        },
        validationSchema: yup.object({
            value: yup
                .number('Enter the value')
                .moreThan(0, 'Value must be greater than 0')
                .required('Value is required'),
            categoryType: yup
                .string('Select the category type')
                .required('Type is required'),
            categoryId: yup
                .number('Select the category')
                .required('Category is required')
        }),
        onSubmit: async (values) => {
            const {value, description, accountId, categoryId, date} = values;

            try {
                toggleLoadingModalOpen();

                await transactionService.newTransaction(
                    accountId,
                    currency(value).intValue,
                    description,
                    moment(date).format('yyyy-MM-DDTHH:mm:ss'),
                    parseInt(categoryId)
                );

                const accounts = await accountService.getAllAccounts();
                dispatch({type: 'setAccounts', payload: accounts});

                toggleLoadingModalOpen();
                navigate(`/transactions/account/${accountId}`);
            } catch (e) {
                if (e.response && e.response.status === 401) {
                    navigate('/');
                }

                toggleLoadingModalOpen();
                showMessageModal('Error', 'An error occurred while processing your request, please try again.');
            }
        },
    });

    const formikTransfer = useFormik({
        initialValues: {
            value: '',
            fromAccountId: accountId,
            toAccountId: '',
            description: '',
            date: moment()

        },
        validationSchema: yup.object({
            value: yup
                .number('Enter the value')
                .moreThan(0, 'Value must be greater than 0')
                .required('Value is required'),
            fromAccountId: yup
                .number('Select the From account')
                .required('From account is required'),
            toAccountId: yup
                .number('Select the To account')
                .test('differentFromAccountId', 'To and From must be different', function (value) {
                    return value !== this.options.parent.fromAccountId;
                })
                .required('To account is required')
        }),
        onSubmit: async (values) => {
            const {value, fromAccountId, toAccountId, description, date} = values;

            try {
                toggleLoadingModalOpen();

                await transferService.newTransfer(
                    currency(value).intValue,
                    description,
                    fromAccountId,
                    toAccountId,
                    moment(date).format('yyyy-MM-DDTHH:mm:ss'),
                );

                const accounts = await accountService.getAllAccounts();
                dispatch({type: 'setAccounts', payload: accounts});

                toggleLoadingModalOpen();
                navigate(`/transactions/account/${accountId}`);
            } catch (e) {
                if (e.response && e.response.status === 401) {
                    navigate('/');
                }

                toggleLoadingModalOpen();
                showMessageModal('Error', 'An error occurred while processing your request, please try again.');
            }
        },
    });

    const onChangeTab = (event, newValue) => {
        navigate(`/transactions/account/${accountId}/new/${tabValueToName(newValue)}`)
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>New Transaction</Typography>
                    <IconButton color='inherit'
                                onClick={currentTab === 0 ? formikTransaction.handleSubmit : formikTransfer.handleSubmit}>
                        <DoneIcon/>
                    </IconButton>
                </Toolbar>
                <Tabs value={currentTab} onChange={onChangeTab} centered>
                    <Tab icon={<AttachMoneyIcon/>} label='Regular'/>
                    <Tab icon={<AutorenewIcon/>} label='Transfer'/>
                </Tabs>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                {
                    currentTab === 0 ? <TransactionForm navigate={navigate} formik={formikTransaction}/> :
                        <TransferForm navigate={navigate} formik={formikTransfer}/>
                }
            </Container>
        </>
    );
};

export default NewTransaction;
