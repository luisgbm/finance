import React, {useContext} from 'react';
import {useFormik} from 'formik';
import * as yup from 'yup';
import TextField from '@mui/material/TextField';
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import {Container, IconButton} from "@mui/material";
import SaveIcon from "@mui/icons-material/Save";
import AppBar from "@mui/material/AppBar";
import {authenticationService} from "../../api/authentication.service";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useDispatch} from "react-redux";
import {useNavigate} from "react-router-dom";

const validationSchema = yup.object({
    userName: yup
        .string('Enter your user name')
        .required('User name is required'),
    password: yup
        .string('Enter your password')
        .min(6, 'Password should have at least 6 characters')
        .required('Password is required'),
});

const NewUser = () => {
    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const navigate = useNavigate();
    const dispatch = useDispatch();

    const formik = useFormik({
        initialValues: {
            userName: '',
            password: '',
        },
        validationSchema: validationSchema,
        onSubmit: async (values) => {
            const {userName, password} = values;

            try {
                toggleLoadingModalOpen();
                const initialData = await authenticationService.newUser(userName, password);
                dispatch({type: 'setAccounts', payload: initialData.accounts});
                dispatch({type: 'setCategories', payload: initialData.categories});
                dispatch({type: 'setScheduledTransactions', payload: initialData.scheduled_transactions});
                toggleLoadingModalOpen();
                navigate('/accounts');
            } catch (e) {
                toggleLoadingModalOpen();

                if (e.response && e.response.status === 409) {
                    showMessageModal('User already exists',
                        `A user with the name "${userName}" already exists, please choose a different user name.`);
                } else {
                    showMessageModal('Error', 'An error occurred while processing your request, please try again.');
                }
            }
        },
    });

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>New User</Typography>
                    <IconButton color='inherit' onClick={formik.handleSubmit}>
                        <SaveIcon/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <form onSubmit={formik.handleSubmit}>
                    <TextField
                        autoCapitalize='none'
                        fullWidth
                        id='userName'
                        name='userName'
                        label='User Name'
                        variant='outlined'
                        autoComplete='off'
                        sx={{mb: 3}}
                        value={formik.values.userName}
                        onChange={formik.handleChange}
                        error={formik.touched.userName && Boolean(formik.errors.userName)}
                        helperText={formik.touched.userName && formik.errors.userName}
                    />
                    <TextField
                        fullWidth
                        id='password'
                        name='password'
                        label='Password'
                        variant='outlined'
                        type='password'
                        sx={{mb: 3}}
                        value={formik.values.password}
                        onChange={formik.handleChange}
                        error={formik.touched.password && Boolean(formik.errors.password)}
                        helperText={formik.touched.password && formik.errors.password}
                    />
                </form>
            </Container>
        </>
    );
};

export default NewUser;
