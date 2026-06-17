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
import PayScheduledTransferForm from "./PayScheduledTransferForm";
import {useDispatch} from "react-redux";
import {accountService} from "../../api/account.service";
import currency from "currency.js";
import {useNavigate, useParams} from "react-router-dom";

const PayScheduledTransfer = () => {
    const params = useParams();
    const navigate = useNavigate();

    const scheduledTransferId = parseInt(params.scheduledTransferId);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const dispatch = useDispatch();

    const formik = useFormik({
        initialValues: {
            originAccountId: '',
            destinationAccountId: '',
            value: '',
            description: '',
            date: moment()

        },
        validationSchema: yup.object({
            value: yup
                .string('Enter the value')
                .required('Value is required'),
            originAccountId: yup
                .string('Select the From account')
                .required('From Account is required'),
            destinationAccountId: yup
                .string('Select the To account')
                .test('differentOriginAccountId', 'To and From must be different', function (value) {
                    return value !== this.options.parent.originAccountId;
                })
                .required('To Account is required'),
        }),
        onSubmit: async (values) => {
            const {originAccountId, destinationAccountId, value, description, date} = values;

            try {
                toggleLoadingModalOpen();

                await scheduledTransactionService.payScheduledTransaction(
                    scheduledTransferId,
                    currency(value).intValue,
                    description,
                    moment(date).format('YYYY-MM-DDTHH:mm:ss'),
                    null,
                    null,
                    originAccountId,
                    destinationAccountId
                );

                const accounts = await accountService.getAllAccounts();
                dispatch({type: 'setAccounts', payload: accounts});

                const scheduledTransactions = await scheduledTransactionService.getAllScheduledTransactions();
                dispatch({type: 'setScheduledTransactions', payload: scheduledTransactions});

                toggleLoadingModalOpen();
                navigate(`/transactions/account/${destinationAccountId}`);
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
                    <Typography variant='h6' sx={{flexGrow: 1}}>Pay Scheduled Transfer</Typography>
                    <IconButton color='inherit'
                                onClick={formik.handleSubmit}>
                        <SaveIcon/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <PayScheduledTransferForm
                    navigate={navigate}
                    formik={formik}
                    scheduledTransferId={scheduledTransferId}
                />
            </Container>
        </>
    );
};

export default PayScheduledTransfer;