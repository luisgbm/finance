import React from 'react';
import CircularProgress from '@mui/material/CircularProgress';
import Backdrop from '@mui/material/Backdrop';

const LoadingModal = (props) => {
    const {open} = props;

    return (
        <Backdrop open={open} sx={{zIndex: theme => theme.zIndex.drawer + 2, color: '#fff'}}>
            <CircularProgress color='inherit'/>
        </Backdrop>
    );
}

export default LoadingModal;
