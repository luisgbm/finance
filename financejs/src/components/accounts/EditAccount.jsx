import React, {useContext, useEffect} from "react";
import {Button, Container, IconButton} from "@mui/material";
import {useFormik} from "formik";
import {accountService} from "../../api/account.service";
import * as yup from "yup";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import TextField from "@mui/material/TextField";
import SaveIcon from "@mui/icons-material/Save";
import DeleteIcon from "@mui/icons-material/Delete";
import LoadingModalContext from "../../context/LoadingModalContext";
import MessageModalContext from "../../context/MessageModalContext";
import {useDispatch, useSelector} from "react-redux";
import {useNavigate, useParams} from "react-router-dom";

const validationSchema = yup.object({
    accountName: yup
        .string('Enter the account name')
        .required('Account name is required')
});

const EditAccount = () => {
    const params = useParams();
    const accountId = parseInt(params.id);
    const accounts = useSelector(state => state.accounts);

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
                const account = await accountService.editAccountById(accountId, accountName);
                dispatch({type: 'editAccount', payload: account});
                toggleLoadingModalOpen();
                navigate('/accounts');
            } catch (e) {
                if (e.response && e.response.status === 401) {
                    navigate('/')
                }

                toggleLoadingModalOpen();
                showMessageModal('Error', 'An error occurred while processing your request, please try again.');
            }
        },
    });

    useEffect(() => {
        (async function loadInitialData() {
            const account = accounts.find(a => a.id === parseInt(accountId));
            await formik.setFieldValue('accountName', account.name);
        })();
        // eslint-disable-next-line
    }, []);

    const onDeleteAccount = async () => {
        try {
            toggleLoadingModalOpen();
            await accountService.deleteAccountById(accountId);
            dispatch({type: 'deleteAccount', payload: accountId});
            toggleLoadingModalOpen();
            navigate('/accounts');
        } catch (e) {
            if (e.response && e.response.status === 401) {
                navigate('/')
            }

            toggleLoadingModalOpen();
            showMessageModal('Error', 'An error occurred while processing your request, please try again.');
        }
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Edit Account</Typography>
                    <IconButton color='inherit' onClick={formik.handleSubmit}>
                        <SaveIcon/>
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
                    <Button
                        fullWidth
                        variant='contained'
                        color='secondary'
                        startIcon={<DeleteIcon/>}
                        size='large'
                        onClick={onDeleteAccount}
                    >
                        Delete
                    </Button>
                </form>
            </Container>
        </>
    );
};

export default EditAccount;
