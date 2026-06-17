import {Container, IconButton} from "@mui/material";
import {useFormik} from "formik";
import moment from "moment";
import * as yup from "yup";
import React, {useContext} from "react";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {scheduledTransactionService} from "../../api/scheduled.transactions.service";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import SaveIcon from "@mui/icons-material/Save";
import AppBar from "@mui/material/AppBar";
import PayScheduledTransactionForm from "./PayScheduledTransactionForm";
import {accountService} from "../../api/account.service";
import {useDispatch} from "react-redux";
import currency from "currency.js";
import {useNavigate, useParams} from "react-router-dom";

const PayScheduledTransaction = () => {
    const params = useParams();
    const navigate = useNavigate();

    const scheduledTransactionId = parseInt(params.scheduledTransactionId);

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
                .string('Enter the value')
                .required('Value is required'),
            categoryType: yup
                .string('Select the category type')
                .required('Type is required'),
            categoryId: yup
                .string('Select the category')
                .required('Category is required')
        }),
        onSubmit: async (values) => {
            const {value, description, accountId, categoryId, date} = values;

            try {
                toggleLoadingModalOpen();

                await scheduledTransactionService.payScheduledTransaction(
                    scheduledTransactionId,
                    currency(value).intValue,
                    description,
                    moment(date).format('YYYY-MM-DDTHH:mm:ss'),
                    parseInt(categoryId),
                    parseInt(accountId),
                    null,
                    null
                );

                const accounts = await accountService.getAllAccounts();
                dispatch({type: 'setAccounts', payload: accounts});

                const scheduledTransactions = await scheduledTransactionService.getAllScheduledTransactions();
                dispatch({type: 'setScheduledTransactions', payload: scheduledTransactions});

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
                    <Typography variant='h6' sx={{flexGrow: 1}}>Pay Scheduled Transaction</Typography>
                    <IconButton color='inherit'
                                onClick={formik.handleSubmit}>
                        <SaveIcon/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <PayScheduledTransactionForm
                    navigate={navigate}
                    formik={formik}
                    scheduledTransactionId={scheduledTransactionId}
                />
            </Container>
        </>
    );
};

export default PayScheduledTransaction;