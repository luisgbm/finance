import {
    Button,
    FormControl,
    FormControlLabel,
    FormHelperText,
    InputLabel,
    MenuItem,
    Select,
    Switch,
    TextField
} from "@mui/material";
import React, {useContext, useEffect} from "react";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {DateTimePicker} from "@mui/x-date-pickers/DateTimePicker";
import RepeatFrequencies from "./RepeatFrequencies";
import moment from "moment";
import DeleteIcon from "@mui/icons-material/Delete";
import {scheduledTransactionService} from "../../api/scheduled.transactions.service";
import {useDispatch, useSelector} from "react-redux";
import currency from "currency.js";


const ScheduledTransferForm = (props) => {
    const {formik, navigate, mode} = props;
    const scheduledTransferId = parseInt(props.scheduledTransferId);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const accounts = useSelector(state => state.accounts);

    const dispatch = useDispatch();

    const onDeleteScheduledTransfer = async () => {
        try {
            toggleLoadingModalOpen();
            await scheduledTransactionService.deleteScheduledTransactionById(scheduledTransferId);
            dispatch({type: 'deleteScheduledTransaction', payload: scheduledTransferId});
            toggleLoadingModalOpen();
            navigate(`/scheduled-transactions`);
        } catch (e) {
            if (e.response && e.response.status === 401) {
                navigate('/');
            }

            toggleLoadingModalOpen();
            showMessageModal('Error', 'An error occurred while processing your request, please try again.');
        }
    };

    useEffect(() => {
        (async function loadInitialData() {
            try {
                toggleLoadingModalOpen();

                if (mode === 'edit') {
                    const scheduledTransfer = await scheduledTransactionService.getScheduledTransactionById(scheduledTransferId);

                    await formik.setFieldValue('originAccountId', scheduledTransfer.origin_account_id);
                    await formik.setFieldValue('destinationAccountId', scheduledTransfer.destination_account_id);
                    await formik.setFieldValue('value', currency(scheduledTransfer.value, {fromCents: true}));
                    await formik.setFieldValue('description', scheduledTransfer.description);
                    await formik.setFieldValue('createdDate', moment(scheduledTransfer.created_date));
                    await formik.setFieldValue('repeat', scheduledTransfer.repeat);

                    if (scheduledTransfer.repeat === true) {
                        await formik.setFieldValue('repeatFreq', scheduledTransfer.repeat_freq);
                        await formik.setFieldValue('repeatInterval', scheduledTransfer.repeat_interval);
                        await formik.setFieldValue('infiniteRepeat', scheduledTransfer.infinite_repeat);

                        if (scheduledTransfer.infinite_repeat === false) {
                            await formik.setFieldValue('endAfterRepeats', scheduledTransfer.end_after_repeats);
                        }
                    }
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
                        value={formik.values.createdDate}
                        onChange={(newValue) => formik.setFieldValue('createdDate', newValue)}
                        format='DD/MM/YYYY HH:mm'
                        slotProps={{
                            textField: {
                                variant: 'outlined',
                                fullWidth: true,
                                error: formik.touched.createdDate && Boolean(formik.errors.createdDate),
                                helperText: formik.touched.createdDate && formik.errors.createdDate
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
            <FormControl
                fullWidth
                variant='outlined'
                sx={{mb: 3}}
                error={formik.touched.repeat && Boolean(formik.errors.repeat)}
            >
                <FormControlLabel
                    control={
                        <Switch
                            checked={formik.values.repeat}
                            onChange={formik.handleChange}
                            name='repeat'
                            id='repeat'
                            color='primary'
                        />
                    }
                    label="Repeat"
                />
            </FormControl>
            <FormControl
                fullWidth
                variant='outlined'
                sx={{mb: 3}}
                error={formik.touched.repeatFreq && Boolean(formik.errors.repeatFreq)}
                disabled={formik.values.repeat === false}
            >
                <InputLabel id='repeatFreq-label'>Frequency</InputLabel>
                <Select
                    labelId='repeatFreq-label'
                    id='repeatFreq'
                    name='repeatFreq'
                    label='Frequency'
                    value={formik.values.repeatFreq}
                    onChange={formik.handleChange}
                >
                    <MenuItem value=''><em>Select...</em></MenuItem>
                    <MenuItem value={RepeatFrequencies.DAYS}>Days</MenuItem>
                    <MenuItem value={RepeatFrequencies.WEEKS}>Weeks</MenuItem>
                    <MenuItem value={RepeatFrequencies.MONTHS}>Months</MenuItem>
                    <MenuItem value={RepeatFrequencies.YEARS}>Years</MenuItem>
                </Select>
                <FormHelperText>{formik.touched.repeatFreq && formik.errors.repeatFreq}</FormHelperText>
            </FormControl>
            <TextField
                id='repeatInterval'
                name='repeatInterval'
                fullWidth
                label='Interval'
                variant='outlined'
                autoComplete='off'
                sx={{mb: 3}}
                value={formik.values.repeatInterval}
                onChange={formik.handleChange}
                error={formik.touched.repeatInterval && Boolean(formik.errors.repeatInterval)}
                helperText={formik.touched.repeatInterval && formik.errors.repeatInterval}
                type='number'
                disabled={formik.values.repeat === false}
            />
            <FormControl
                fullWidth
                variant='outlined'
                sx={{mb: 3}}
                error={formik.touched.infiniteRepeat && Boolean(formik.errors.infiniteRepeat)}
                disabled={formik.values.repeat === false}
            >
                <FormControlLabel
                    control={
                        <Switch
                            checked={formik.values.infiniteRepeat}
                            onChange={formik.handleChange}
                            name='infiniteRepeat'
                            id='infiniteRepeat'
                            color='primary'
                        />
                    }
                    label="Infinite Repeat"
                />
            </FormControl>
            <TextField
                id='endAfterRepeats'
                name='endAfterRepeats'
                fullWidth
                label='End After Repetitions'
                variant='outlined'
                autoComplete='off'
                sx={{mb: 3}}
                value={formik.values.endAfterRepeats}
                onChange={formik.handleChange}
                error={formik.touched.endAfterRepeats && Boolean(formik.errors.endAfterRepeats)}
                helperText={formik.touched.endAfterRepeats && formik.errors.endAfterRepeats}
                type='number'
                disabled={formik.values.repeat === false || formik.values.infiniteRepeat === true}
            />
            {
                mode === 'edit' ? <Button
                    fullWidth
                    variant='contained'
                    color='secondary'
                    startIcon={<DeleteIcon/>}
                    size='large'
                    onClick={onDeleteScheduledTransfer}
                >
                    Delete
                </Button> : <></>
            }
        </>
    );
};

export default ScheduledTransferForm;