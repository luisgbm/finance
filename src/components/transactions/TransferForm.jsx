import {
    Button,
    FormControl,
    FormHelperText,
    InputLabel,
    MenuItem,
    Select,
    TextField
} from "@mui/material";
import {DateTimePicker} from "@mui/x-date-pickers/DateTimePicker";
import React, {useContext, useEffect} from "react";
import DeleteIcon from "@mui/icons-material/Delete";
import {transferService} from "../../api/transfer.service";
import moment from "moment";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useDispatch, useSelector} from "react-redux";
import {accountService} from "../../api/account.service";
import currency from 'currency.js';

const TransferForm = (props) => {
    const accounts = useSelector(state => state.accounts);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const dispatch = useDispatch();

    const {formik, navigate, mode, transferId, fromAccountId} = props;

    useEffect(() => {
        (async function loadInitialData() {
            try {
                toggleLoadingModalOpen();

                if (mode === 'edit') {
                    const transfer = await transferService.getTransferById(transferId);

                    formik.setFieldValue('value', currency(transfer.data.value, {fromCents: true}));
                    formik.setFieldValue('description', transfer.data.description);
                    formik.setFieldValue('date', moment(transfer.data.date));
                    formik.setFieldValue('fromAccountId', transfer.data.origin_account);
                    formik.setFieldValue('toAccountId', transfer.data.destination_account);
                }

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

    const onDeleteTransfer = async () => {
        try {
            toggleLoadingModalOpen();

            await transferService.deleteTransferById(transferId);

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
    };

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
                error={formik.touched.toAccountId && Boolean(formik.errors.toAccountId)}
            >
                <InputLabel id='toAccountId-label'>To</InputLabel>
                <Select
                    labelId='toAccountId-label'
                    id='toAccountId'
                    name='toAccountId'
                    label='To'
                    value={formik.values.toAccountId}
                    onChange={formik.handleChange}
                >
                    {
                        accounts.map(account =>
                            <MenuItem value={account.id} key={account.id}>{account.name}</MenuItem>
                        )
                    }
                </Select>
                <FormHelperText>{formik.touched.toAccountId && formik.errors.toAccountId}</FormHelperText>
            </FormControl>
            <FormControl
                fullWidth
                variant='outlined'
                sx={{mb: 3}}
                error={formik.touched.fromAccountId && Boolean(formik.errors.fromAccountId)}
            >
                <InputLabel id='fromAccountId-label'>From</InputLabel>
                <Select
                    labelId='fromAccountId-label'
                    id='fromAccountId'
                    name='fromAccountId'
                    label='From'
                    value={formik.values.fromAccountId}
                    onChange={formik.handleChange}
                >
                    {
                        accounts.map(account =>
                            <MenuItem value={account.id} key={account.id}>{account.name}</MenuItem>
                        )
                    }
                </Select>
                <FormHelperText>{formik.touched.fromAccountId && formik.errors.fromAccountId}</FormHelperText>
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
            {
                mode === 'edit' ? <Button
                    fullWidth
                    variant='contained'
                    color='secondary'
                    startIcon={<DeleteIcon/>}
                    size='large'
                    onClick={onDeleteTransfer}
                >
                    Delete
                </Button> : <></>
            }
        </>
    );
};

export default TransferForm;
