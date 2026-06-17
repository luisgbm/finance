import React, {useContext, useEffect} from 'react';

import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import AppBar from '@mui/material/AppBar';
import SaveIcon from '@mui/icons-material/Save';
import {
    Button,
    Container,
    FormControl,
    FormHelperText,
    IconButton,
    InputLabel,
    MenuItem,
    Select
} from '@mui/material';
import DeleteIcon from '@mui/icons-material/Delete';
import {categoryService} from "../../api/category.service";
import * as yup from "yup";
import {useFormik} from "formik";
import CategoryTypes from "./CategoryTypes";
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

const EditCategory = () => {
    const params = useParams();
    const categoryId = parseInt(params.id);

    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const navigate = useNavigate();
    const dispatch = useDispatch();

    const formik = useFormik({
        initialValues: {
            categoryName: '',
            categoryType: ''
        },
        validationSchema: validationSchema,
        onSubmit: async (values) => {
            const {categoryName, categoryType} = values;

            try {
                toggleLoadingModalOpen();
                const category = await categoryService.editCategoryById(categoryId, categoryName, categoryType);
                dispatch({type: 'editCategory', payload: category});
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

    const onDeleteCategory = async () => {
        try {
            toggleLoadingModalOpen();
            await categoryService.deleteCategoryById(categoryId);
            dispatch({type: 'deleteCategory', payload: categoryId});
            toggleLoadingModalOpen();
            navigate('/categories');
        } catch (e) {
            if (e.response && e.response.status === 401) {
                navigate('/')
            }

            toggleLoadingModalOpen();
            showMessageModal('Error', 'An error occurred while processing your request, please try again.');
        }
    }

    useEffect(() => {
        (async function loadCategoryData() {
            try {
                toggleLoadingModalOpen();
                const category = await categoryService.getCategoryById(categoryId);
                formik.values.categoryName = category.data.name;
                formik.values.categoryType = category.data.categorytype;
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
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Edit Category</Typography>
                    <IconButton color='inherit' onClick={formik.handleSubmit}>
                        <SaveIcon/>
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
                    helperText={formik.touched.categoryType && formik.errors.categoryType}
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
                <Button
                    fullWidth
                    variant='contained'
                    color='secondary'
                    startIcon={<DeleteIcon/>}
                    size='large'
                    onClick={onDeleteCategory}
                    sx={{mt: 3}}
                >
                    Delete
                </Button>
            </Container>
        </>
    );
}

export default EditCategory;
