import {FormControl, FormHelperText, InputLabel, MenuItem, Select, TextField} from "@mui/material";
import {DateTimePicker} from "@mui/x-date-pickers/DateTimePicker";
import React, {useContext, useEffect} from "react";
import moment from "moment";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useSelector} from "react-redux";
import currency from "currency.js";


const PayScheduledTransferForm = (props) => {
    const {formik, navigate} = props;
    const scheduledTransferId = parseInt(props.scheduledTransferId);
    const allscheduledTransactions = useSelector(state => state.scheduledTransactions);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const accounts = useSelector(state => state.accounts);


    useEffect(() => {
        (async function loadInitialData() {
            try {
                toggleLoadingModalOpen();

                const scheduledTransaction = allscheduledTransactions.find(scheduledTransaction => scheduledTransaction.id === scheduledTransferId);

                await formik.setFieldValue('originAccountId', scheduledTransaction.origin_account_id);
                await formik.setFieldValue('destinationAccountId', scheduledTransaction.destination_account_id);
                await formik.setFieldValue('value', currency(scheduledTransaction.value, {fromCents: true}));
                await formik.setFieldValue('description', scheduledTransaction.description);
                await formik.setFieldValue('date', moment(scheduledTransaction.next_date));

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

    return (
        <>
            <TextField
                fullWidth
                id='value'
                name='value'
                label='Value'
                variant='outlined'
                autoComplete='off'
                sx={{mb: 3}}
                type='number'
                value={formik.values.value}
                onChange={formik.handleChange}
                error={formik.touched.value && Boolean(formik.errors.value)}
                helperText={formik.touched.value && formik.errors.value}
            />
            <FormControl
                fullWidth
                variant='outlined'
                sx={{mb: 3}}
                error={formik.touched.destinationAccountId && Boolean(formik.errors.destinationAccountId)}
            >
                <InputLabel id='destinationAccountId-label'>To</InputLabel>
                <Select
                    labelId='destinationAccountId-label'
                    id='destinationAccountId'
                    name='destinationAccountId'
                    label='To'
                    value={formik.values.destinationAccountId}
                    onChange={formik.handleChange}
                >
                    {
                        accounts.map(account =>
                            <MenuItem value={account.id} key={account.id}>{account.name}</MenuItem>
                        )
                    }
                </Select>
                <FormHelperText>{formik.touched.destinationAccountId && formik.errors.destinationAccountId}</FormHelperText>
            </FormControl>
            <FormControl
                fullWidth
                variant='outlined'
                sx={{mb: 3}}
                error={formik.touched.originAccountId && Boolean(formik.errors.originAccountId)}
            >
                <InputLabel id='originAccountId-label'>From</InputLabel>
                <Select
                    labelId='originAccountId-label'
                    id='originAccountId'
                    name='originAccountId'
                    label='From'
                    value={formik.values.originAccountId}
                    onChange={formik.handleChange}
                >
                    {
                        accounts.map(account =>
                            <MenuItem value={account.id} key={account.id}>{account.name}</MenuItem>
                        )
                    }
                </Select>
                <FormHelperText>{formik.touched.originAccountId && formik.errors.originAccountId}</FormHelperText>
            </FormControl>
            <FormControl
                fullWidth
                sx={{mb: 3}}
            >
                <DateTimePicker
                        label='Date/Time'
                        value={formik.values.date}
                        onChange={(newValue) => formik.setFieldValue('date', newValue)}
                        format='DD/MM/YYYY HH:mm'
                        slotProps={{
                            textField: {
                                variant: 'outlined',
                                fullWidth: true,
                                error: formik.touched.date && Boolean(formik.errors.date),
                                helperText: formik.touched.date && formik.errors.date
                            }
                        }}
                />
            </FormControl>
            <TextField
                id='description'
                name='description'
                fullWidth
                label='Description'
                variant='outlined'
                autoComplete='off'
                sx={{mb: 3}}
                value={formik.values.description}
                onChange={formik.handleChange}
                error={formik.touched.description && Boolean(formik.errors.description)}
                helperText={formik.touched.description && formik.errors.description}
            />
        </>
    );
};

export default PayScheduledTransferForm;