import {Container, IconButton, Tab, Tabs} from "@mui/material";
import React, {useContext} from "react";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import AppBar from "@mui/material/AppBar";
import DoneIcon from "@mui/icons-material/Done";
import ScheduledTransactionForm from "./ScheduledTransactionForm";
import {useFormik} from "formik";
import {scheduledTransactionService} from "../../api/scheduled.transactions.service";
import {
    scheduledTransactionInitialValues,
    scheduledTransactionValidationSchema
} from "./ScheduledTransactionFormParams";
import {scheduledTransferInitialValues, scheduledTransferValidationSchema} from "./ScheduledTransferFormParams";
import AttachMoneyIcon from "@mui/icons-material/AttachMoney";
import AutorenewIcon from "@mui/icons-material/Autorenew";
import ScheduledTransferForm from "./ScheduledTransferForm";
import moment from "moment";
import {useDispatch} from "react-redux";
import currency from "currency.js";
import {useNavigate, useParams} from "react-router-dom";

const NewScheduledTransaction = () => {
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

    const onChangeTab = (event, newValue) => {
        navigate(`/scheduled-transactions/new/${tabValueToName(newValue)}`)
    };

    const currentTab = tabNameToValue(params.type);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const dispatch = useDispatch();

    const formikScheduledTransfer = useFormik({
        initialValues: scheduledTransferInitialValues,
        validationSchema: scheduledTransferValidationSchema,
        onSubmit: async (values) => {
            const {
                originAccountId,
                destinationAccountId,
                value,
                description,
                createdDate,
                repeat,
                repeatFreq,
                repeatInterval,
                infiniteRepeat,
                endAfterRepeats
            } = values;

            try {
                toggleLoadingModalOpen();

                let scheduledTransaction = await scheduledTransactionService.newScheduledTransaction(
                    'Transfer',
                    null,
                    currency(value).intValue,
                    description,
                    null,
                    parseInt(originAccountId),
                    parseInt(destinationAccountId),
                    moment(createdDate).format('yyyy-MM-DDTHH:mm:ss'),
                    repeat,
                    repeatFreq,
                    parseInt(repeatInterval),
                    infiniteRepeat,
                    parseInt(endAfterRepeats)
                );

                dispatch({type: 'addScheduledTransaction', payload: scheduledTransaction});

                toggleLoadingModalOpen();
                navigate(`/scheduled-transactions`);
            } catch (e) {
                if (e.response && e.response.status === 401) {
                    navigate('/');
                }

                toggleLoadingModalOpen();
                showMessageModal('Error', 'An error occurred while processing your request, please try again.');
            }
        },
    });

    const formikScheduledTransaction = useFormik({
        initialValues: scheduledTransactionInitialValues,
        validationSchema: scheduledTransactionValidationSchema,
        onSubmit: async (values) => {
            const {
                accountId,
                value,
                description,
                categoryId,
                createdDate,
                repeat,
                repeatFreq,
                repeatInterval,
                infiniteRepeat,
                endAfterRepeats
            } = values;

            try {
                toggleLoadingModalOpen();

                let scheduledTransaction = await scheduledTransactionService.newScheduledTransaction(
                    'Transaction',
                    parseInt(accountId),
                    currency(value).intValue,
                    description,
                    parseInt(categoryId),
                    null,
                    null,
                    moment(createdDate).format('yyyy-MM-DDTHH:mm:ss'),
                    repeat,
                    repeatFreq,
                    parseInt(repeatInterval),
                    infiniteRepeat,
                    parseInt(endAfterRepeats)
                );

                dispatch({type: 'addScheduledTransaction', payload: scheduledTransaction});

                toggleLoadingModalOpen();
                navigate(`/scheduled-transactions`);
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
                    <Typography variant='h6' sx={{flexGrow: 1}}>New Scheduled Transaction</Typography>
                    <IconButton color='inherit'
                                onClick={currentTab === 0 ? formikScheduledTransaction.handleSubmit : formikScheduledTransfer.handleSubmit}>
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
                    currentTab === 0 ?
                        <ScheduledTransactionForm navigate={navigate} formik={formikScheduledTransaction}/> :
                        <ScheduledTransferForm navigate={navigate} formik={formikScheduledTransfer}/>
                }
            </Container>
        </>
    );
};

export default NewScheduledTransaction;