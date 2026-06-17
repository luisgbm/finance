import React, {useContext} from 'react';

import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import AppBar from '@mui/material/AppBar';
import SaveIcon from '@mui/icons-material/Save';
import {Container, IconButton} from '@mui/material';
import moment from 'moment';
import {transferService} from "../../api/transfer.service";
import {useFormik} from "formik";
import * as yup from "yup";
import TransferForm from "./TransferForm";
import {accountService} from "../../api/account.service";
import {useDispatch} from "react-redux";
import currency from "currency.js";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useNavigate, useParams} from "react-router-dom";

const EditTransfer = () => {
    const params = useParams();
    const navigate = useNavigate();
    const {transferId, fromAccountId} = params;

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const dispatch = useDispatch();

    const formik = useFormik({
        initialValues: {
            value: '',
            fromAccountId: '',
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

                await transferService.editTransferById(
                    transferId,
                    currency(value).intValue,
                    description,
                    moment(date).format('YYYY-MM-DDTHH:mm:ss'),
                    fromAccountId,
                    toAccountId
                );

                const accounts = await accountService.getAllAccounts();
                dispatch({type: 'setAccounts', payload: accounts});

                toggleLoadingModalOpen();
                navigate(`/transactions/account/${fromAccountId}`);
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
                    <Typography variant='h6' sx={{flexGrow: 1}}>Edit Transfer</Typography>
                    <IconButton color='inherit'
                                onClick={formik.handleSubmit}>
                        <SaveIcon/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <TransferForm formik={formik} navigate={navigate} mode='edit' transferId={transferId}
                              fromAccountId={fromAccountId}/>
            </Container>
        </>
    );
};

export default EditTransfer;
