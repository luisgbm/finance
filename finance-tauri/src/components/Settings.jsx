import React from 'react';
import {authenticationService} from '../api/authentication.service';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import {Button, Container} from '@mui/material';
import ExitToAppIcon from '@mui/icons-material/ExitToApp';
import LoadingModal from "./LoadingModal";
import {useNavigate} from "react-router-dom";

const Settings = () => {
    const [loadingModalOpen, setLoadingModalOpen] = React.useState(false);

    const navigate = useNavigate();

    const onLogout = () => {
        setLoadingModalOpen(true);
        authenticationService.logout();
        setLoadingModalOpen(false);
        navigate('/');
    };

    return (
        <>
            <LoadingModal
                open={loadingModalOpen}
            />
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Settings</Typography>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                <Button
                    fullWidth
                    variant='contained'
                    color='secondary'
                    startIcon={<ExitToAppIcon/>}
                    size='large'
                    onClick={onLogout}
                >
                    Logout
                </Button>
            </Container>
        </>
    );
};

export default Settings;
