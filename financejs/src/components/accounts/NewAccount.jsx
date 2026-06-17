import React, {useContext} from "react";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import {Container, IconButton} from "@mui/material";
import AppBar from "@mui/material/AppBar";
import DoneIcon from "@mui/icons-material/Done";
import {useFormik} from "formik";
import * as yup from "yup";
import {accountService} from "../../api/account.service";
import TextField from "@mui/material/TextField";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useDispatch} from "react-redux";
import {useNavigate} from "react-router-dom";

const validationSchema = yup.object({
    accountName: yup
        .string('Enter the account name')
        .required('Account name is required')
});

const NewAccount = () => {
    const toggleLoadingModalOpen = useContext(LoadingModalContext);
    const {showMessageModal} = useContext(MessageModalContext);

    const navigate = useNavigate();
    const dispatch = useDispatch();

    const formik = useFormik({
        initialValues: {
            accountName: ''
        },
        validationSchema: validationSchema,
        onSubmit: async (values) => {
            const {accountName} = values;

            try {
                toggleLoadingModalOpen();
                const newAccount = await accountService.newAccount(accountName);
                dispatch({type: 'addAccount', payload: newAccount});
                toggleLoadingModalOpen();
                navigate('/accounts');
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
                    <Typography variant='h6' sx={{flexGrow: 1}}>New Account</Typography>
                    <IconButton color='inherit' onClick={formik.handleSubmit}>
                        <DoneIcon/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <form onSubmit={formik.handleSubmit}>
                    <TextField
                        fullWidth
                        id='accountName'
                        name='accountName'
                        label='Account Name'
                        variant='outlined'
                        autoComplete='off'
                        sx={{mb: 3}}
                        value={formik.values.accountName}
                        onChange={formik.handleChange}
                        error={formik.touched.accountName && Boolean(formik.errors.accountName)}
                        helperText={formik.touched.accountName && formik.errors.accountName}
                    />
                </form>
            </Container>
        </>
    );
};

export default NewAccount;
