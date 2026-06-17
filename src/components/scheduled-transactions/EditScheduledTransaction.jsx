import React, {useContext} from "react";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {Container, IconButton} from "@mui/material";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import DoneIcon from "@mui/icons-material/Done";
import ScheduledTransactionForm from "./ScheduledTransactionForm";
import {useFormik} from "formik";
import {
    scheduledTransactionInitialValues,
    scheduledTransactionValidationSchema
} from "./ScheduledTransactionFormParams";
import {scheduledTransactionService} from "../../api/scheduled.transactions.service";
import moment from "moment";
import {useDispatch} from "react-redux";
import currency from "currency.js";
import {useNavigate, useParams} from "react-router-dom";

const EditScheduledTransaction = () => {
    const params = useParams();
    const navigate = useNavigate();

    const scheduledTransactionId = parseInt(params.scheduledTransactionId);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const dispatch = useDispatch();

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

                const scheduledTransaction = await scheduledTransactionService.editScheduledTransactionById(
                    scheduledTransactionId,
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

                dispatch({type: 'editScheduledTransaction', payload: scheduledTransaction});

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
                    <Typography variant='h6' sx={{flexGrow: 1}}>Edit Scheduled Transaction</Typography>
                    <IconButton color='inherit'
                                onClick={formikScheduledTransaction.handleSubmit}>
                        <DoneIcon/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <ScheduledTransactionForm
                    navigate={navigate}
                    formik={formikScheduledTransaction}
                    mode='edit'
                    scheduledTransactionId={scheduledTransactionId}
                />
            </Container>
        </>
    );
};

export default EditScheduledTransaction;