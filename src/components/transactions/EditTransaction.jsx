import React, {useContext} from 'react';

import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import AppBar from '@mui/material/AppBar';
import SaveIcon from '@mui/icons-material/Save';
import {Container, IconButton} from '@mui/material';
import moment from 'moment';
import {transactionService} from "../../api/transaction.service";
import {useFormik} from "formik";
import * as yup from "yup";
import TransactionForm from "./TransactionForm";
import {accountService} from "../../api/account.service";
import {useDispatch} from "react-redux";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import currency from "currency.js";
import {useNavigate, useParams} from "react-router-dom";

const EditTransaction = () => {
    const params = useParams();
    const navigate = useNavigate();
    const transactionId = parseInt(params.transactionId);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const dispatch = useDispatch();

    const formik = useFormik({
        initialValues: {
            value: '',
            description: '',
            accountId: '',
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

                await transactionService.editTransactionById(
                    transactionId,
                    currency(value).intValue,
                    description,
                    moment(date).format('YYYY-MM-DDTHH:mm:ss'),
                    parseInt(accountId),
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

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Edit Transaction</Typography>
                    <IconButton color='inherit'
                                onClick={formik.handleSubmit}>
                        <SaveIcon/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <TransactionForm
                    formik={formik}
                    mode='edit'
                    transactionId={transactionId}
                    navigate={navigate}
                />
            </Container>
        </>
    );
};

export default EditTransaction;
