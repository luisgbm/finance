import React, {useContext} from "react";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {Container, IconButton} from "@mui/material";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import DoneIcon from "@mui/icons-material/Done";
import ScheduledTransferForm from "./ScheduledTransferForm";
import {useFormik} from "formik";
import {scheduledTransferInitialValues, scheduledTransferValidationSchema} from "./ScheduledTransferFormParams";
import {scheduledTransactionService} from "../../api/scheduled.transactions.service";
import moment from "moment";
import {useDispatch} from "react-redux";
import currency from "currency.js";
import {useNavigate, useParams} from "react-router-dom";

const EditScheduledTransfer = () => {
    const params = useParams();
    const navigate = useNavigate();

    const {scheduledTransferId} = params;

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

                const scheduledTransaction = await scheduledTransactionService.editScheduledTransactionById(
                    scheduledTransferId,
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
                    <Typography variant='h6' sx={{flexGrow: 1}}>Edit Scheduled Transfer</Typography>
                    <IconButton color='inherit'
                                onClick={formikScheduledTransfer.handleSubmit}>
                        <DoneIcon/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <ScheduledTransferForm
                    navigate={navigate}
                    formik={formikScheduledTransfer}
                    mode='edit'
                    scheduledTransferId={scheduledTransferId}
                />
            </Container>
        </>
    );
};

export default EditScheduledTransfer;