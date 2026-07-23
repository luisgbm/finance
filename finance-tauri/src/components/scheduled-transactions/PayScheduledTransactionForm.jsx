import {FormControl, FormHelperText, InputLabel, MenuItem, Select, TextField} from "@mui/material";
import CategoryTypes from "../categories/CategoryTypes";
import {DateTimePicker} from "@mui/x-date-pickers/DateTimePicker";
import React, {useContext, useEffect} from "react";
import moment from "moment";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useSelector} from "react-redux";
import currency from "currency.js";


const PayScheduledTransactionForm = (props) => {
    const {formik, navigate} = props;
    const scheduledTransactionId = parseInt(props.scheduledTransactionId);
    const allscheduledTransactions = useSelector(state => state.scheduledTransactions);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const accounts = useSelector(state => state.accounts);
    const allCategories = useSelector(state => state.categories);

    const [categories, setCategories] = React.useState([]);


    const updateCategories = async (categoryType) => {
        await formik.setFieldValue('categoryType', categoryType, true);

        if (categoryType !== '') {
            setCategories(allCategories.filter(category => category.categorytype === categoryType));
        }

        await formik.setFieldValue('categoryId', '', true);
    };

    useEffect(() => {
        (async function loadInitialData() {
            try {
                toggleLoadingModalOpen();

                const scheduledTransaction = allscheduledTransactions.find(scheduledTransaction => scheduledTransaction.id === scheduledTransactionId);

                setCategories(allCategories.filter(category => category.categorytype === scheduledTransaction.category_type));

                await formik.setFieldValue('value', currency(scheduledTransaction.value, {fromCents: true}));
                await formik.setFieldValue('description', scheduledTransaction.description);
                await formik.setFieldValue('accountId', scheduledTransaction.account_id);
                await formik.setFieldValue('categoryType', scheduledTransaction.category_type);
                await formik.setFieldValue('categoryId', scheduledTransaction.category_id);
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
                error={formik.touched.categoryType && Boolean(formik.errors.categoryType)}
            >
                <InputLabel id='categoryType-label'>Type</InputLabel>
                <Select
                    labelId='categoryType-label'
                    id='categoryType'
                    name='categoryType'
                    label='Type'
                    value={formik.values.categoryType}
                    onChange={e => updateCategories(e.target.value)}
                >
                    <MenuItem value=''><em>Select...</em></MenuItem>
                    <MenuItem value={CategoryTypes.EXPENSE}>Expense</MenuItem>
                    <MenuItem value={CategoryTypes.INCOME}>Income</MenuItem>
                </Select>
                <FormHelperText>{formik.touched.categoryType && formik.errors.categoryType}</FormHelperText>
            </FormControl>
            <FormControl
                disabled={formik.values.categoryType === ''}
                fullWidth
                variant='outlined'
                sx={{mb: 3}}
                error={formik.touched.categoryId && Boolean(formik.errors.categoryId)}
            >
                <InputLabel id='categoryId-label'>Category</InputLabel>
                <Select
                    labelId='categoryId-label'
                    id='categoryId'
                    name='categoryId'
                    label='Category'
                    value={formik.values.categoryId}
                    onChange={formik.handleChange}
                >
                    <MenuItem value=''><em>Select...</em></MenuItem>
                    {
                        categories.map(category =>
                            <MenuItem value={category.id} key={category.id}>{category.name}</MenuItem>
                        )
                    }
                </Select>
                <FormHelperText>{formik.touched.categoryId && formik.errors.categoryId}</FormHelperText>
            </FormControl>
            <FormControl
                fullWidth
                variant='outlined'
                sx={{mb: 3}}
                error={formik.touched.accountId && Boolean(formik.errors.accountId)}
            >
                <InputLabel id='accountId-label'>Account</InputLabel>
                <Select
                    labelId='accountId-label'
                    id='accountId'
                    name='accountId'
                    label='Account'
                    value={formik.values.accountId}
                    onChange={formik.handleChange}
                >
                    {
                        accounts.map(account =>
                            <MenuItem value={account.id} key={account.id}>{account.name}</MenuItem>
                        )
                    }
                </Select>
                <FormHelperText>{formik.touched.accountId && formik.errors.accountId}</FormHelperText>
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

export default PayScheduledTransactionForm;