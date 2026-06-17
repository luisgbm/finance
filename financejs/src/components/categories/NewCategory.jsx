import React, {useContext} from 'react';

import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import AppBar from '@mui/material/AppBar';
import {
    Container,
    FormControl,
    FormHelperText,
    IconButton,
    InputLabel,
    MenuItem,
    Select
} from '@mui/material';
import {Done} from '@mui/icons-material';
import CategoryTypes from './CategoryTypes';
import {categoryService} from "../../api/category.service";
import {useFormik} from "formik";
import * as yup from "yup";
import TextField from "@mui/material/TextField";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useDispatch} from "react-redux";
import {useNavigate, useParams} from "react-router-dom";

const validationSchema = yup.object({
    categoryName: yup
        .string('Enter the category name')
        .required('Category name is required'),
    categoryType: yup
        .string('Select the category type')
        .required('Category type is required')
});

const NewCategory = () => {
    const params = useParams();

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const navigate = useNavigate();
    const dispatch = useDispatch();

    const formik = useFormik({
        initialValues: {
            categoryName: '',
            categoryType: params.type === 'expense' ? CategoryTypes.EXPENSE : CategoryTypes.INCOME
        },
        validationSchema: validationSchema,
        onSubmit: async (values) => {
            const {categoryName, categoryType} = values;

            try {
                toggleLoadingModalOpen();
                const newCategory = await categoryService.newCategory(categoryName, categoryType);
                dispatch({type: 'addCategory', payload: newCategory});
                toggleLoadingModalOpen();
                navigate(`/categories/${categoryType.toLowerCase()}`);
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
                    <Typography variant='h6' sx={{flexGrow: 1}}>New Category</Typography>
                    <IconButton color='inherit' onClick={formik.handleSubmit}>
                        <Done/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <TextField
                    fullWidth
                    id='categoryName'
                    name='categoryName'
                    label='Category Name'
                    variant='outlined'
                    autoComplete='off'
                    sx={{mb: 3}}
                    value={formik.values.categoryName}
                    onChange={formik.handleChange}
                    error={formik.touched.categoryName && Boolean(formik.errors.categoryName)}
                    helperText={formik.touched.categoryName && formik.errors.categoryName}
                />
                <FormControl
                    fullWidth
                    variant='outlined'
                    error={formik.touched.categoryType && Boolean(formik.errors.categoryType)}
                >
                    <InputLabel>Category Type</InputLabel>
                    <Select
                        id='categoryType'
                        name='categoryType'
                        label='Category Type'
                        value={formik.values.categoryType}
                        onChange={formik.handleChange}
                    >
                        <MenuItem value={CategoryTypes.EXPENSE}>Expense</MenuItem>
                        <MenuItem value={CategoryTypes.INCOME}>Income</MenuItem>
                    </Select>
                    <FormHelperText>{formik.touched.categoryType && formik.errors.categoryType}</FormHelperText>
                </FormControl>
            </Container>
        </>
    );
};

export default NewCategory;
