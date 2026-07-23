import React, {useContext, useEffect} from 'react';
import {Button, Container, IconButton} from "@mui/material";
import * as yup from "yup";
import {useFormik} from "formik";
import {authenticationService} from "../../api/authentication.service";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import AppBar from "@mui/material/AppBar";
import {Add} from "@mui/icons-material";
import TextField from "@mui/material/TextField";
import VpnKeyIcon from "@mui/icons-material/VpnKey";
import {useDispatch} from "react-redux";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useNavigate} from "react-router-dom";

const validationSchema = yup.object({
    userName: yup
        .string('Enter your user name')
        .required('User name is required'),
    password: yup
        .string('Enter your password')
        .required('Password is required'),
});

const Login = () => {
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
        onSubmit: async (values, {resetForm}) => {
            const {userName, password} = values;

            try {
                toggleLoadingModalOpen();
                const login = await authenticationService.login(userName, password);
                dispatch({type: 'setAccounts', payload: login.accounts});
                dispatch({type: 'setCategories', payload: login.categories});
                dispatch({type: 'setScheduledTransactions', payload: login.scheduled_transactions});
                toggleLoadingModalOpen();
                navigate('/accounts');
            } catch (e) {
                resetForm();

                toggleLoadingModalOpen();

                if (e.response && e.response.status === 401) {
                    showMessageModal('Login failed', 'Wrong user name or password, please try again.');
                } else {
                    showMessageModal('Error', 'An error occurred while processing your request, please try again.');
                }
            }
        },
    });

    const onNewUser = () => {
        navigate('/users/new');
    };

    useEffect(() => {
        (async function checkToken() {
            toggleLoadingModalOpen();

            const token = await authenticationService.validateToken();

            if (token && token.data) {
                dispatch({type: 'setAccounts', payload: token.data.accounts});
                dispatch({type: 'setCategories', payload: token.data.categories});
                dispatch({type: 'setScheduledTransactions', payload: token.data.scheduled_transactions});
                navigate('/accounts');
            }

            toggleLoadingModalOpen();
        })();
    }, []);  // eslint-disable-line react-hooks/exhaustive-deps

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Login</Typography>
                    <IconButton color='inherit' onClick={onNewUser}>
                        <Add/>
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
                    <Button
                        type='submit'
                        fullWidth
                        variant='contained'
                        color='primary'
                        startIcon={<VpnKeyIcon/>}
                        size='large'
                        sx={{mb: 3}}
                    >
                        Login
                    </Button>
                </form>
            </Container>
        </>
    );
};

export default Login;
