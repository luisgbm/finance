import React from 'react';
import NewAccount from './accounts/NewAccount';
import EditAccount from './accounts/EditAccount';
import CategoryList from './categories/CategoryList';
import NewCategory from './categories/NewCategory';
import EditCategory from './categories/EditCategory';
import TransactionList from './transactions/TransactionList';
import NewTransaction from './transactions/NewTransaction';
import EditTransaction from './transactions/EditTransaction';
import CssBaseline from '@mui/material/CssBaseline';
import {BrowserRouter as Router, Route, Routes} from 'react-router-dom';
import AccountList from './accounts/AccountList';
import {createTheme, ThemeProvider} from '@mui/material/styles';
import {LocalizationProvider} from '@mui/x-date-pickers/LocalizationProvider';
import {AdapterMoment} from '@mui/x-date-pickers/AdapterMoment';
import Settings from './Settings';
import EditTransfer from "./transactions/EditTransfer";
import BottomNavBar from "./BottomNavBar";
import ScheduledTransactionsList from "./scheduled-transactions/ScheduledTransactionsList";
import NewScheduledTransaction from "./scheduled-transactions/NewScheduledTransaction";
import LoadingModal from "./LoadingModal";
import LoadingModalContext from "../context/LoadingModalContext";
import MessageModalContext from "../context/MessageModalContext";
import MessageModal from "./MessageModal";
import EditScheduledTransaction from "./scheduled-transactions/EditScheduledTransaction";
import PayScheduledTransaction from "./scheduled-transactions/PayScheduledTransaction";
import EditScheduledTransfer from "./scheduled-transactions/EditScheduledTransfer";
import PayScheduledTransfer from "./scheduled-transactions/PayScheduledTransfer";
import {useDispatch} from "react-redux";
import {initialDataService} from "../api/initial.data.service";

const theme = createTheme({
    // Replicate the Material-UI v4 default theme so every component matches the original
    // look. MUI v5+ changed the default palette and several component defaults; the
    // overrides below restore the v4 values app-wide.
    palette: {
        mode: 'light',
        primary: {main: '#3f51b5', light: '#7986cb', dark: '#303f9f', contrastText: '#fff'},
        secondary: {main: '#f50057', light: '#ff4081', dark: '#c51162', contrastText: '#fff'},
        error: {main: '#f44336', light: '#e57373', dark: '#d32f2f', contrastText: '#fff'},
        warning: {main: '#ff9800', light: '#ffb74d', dark: '#f57c00', contrastText: 'rgba(0, 0, 0, 0.87)'},
        info: {main: '#2196f3', light: '#64b5f6', dark: '#1976d2', contrastText: '#fff'},
        success: {main: '#4caf50', light: '#81c784', dark: '#388e3c', contrastText: 'rgba(0, 0, 0, 0.87)'},
    },
    components: {
        // v4 Tabs defaulted to textColor="inherit" and indicatorColor="secondary"
        // (v5+ default to "primary", which is invisible on a colored app bar).
        MuiTabs: {
            defaultProps: {
                textColor: 'inherit',
                indicatorColor: 'secondary',
            },
        },
        // v4 filled Chips used an opaque grey[300] background; v5+ uses a translucent
        // overlay that disappears on the colored app bar (making the balance look like
        // plain black text).
        MuiChip: {
            styleOverrides: {
                root: ({ownerState}) => ({
                    ...(((ownerState.variant === 'filled' || !ownerState.variant) &&
                        (!ownerState.color || ownerState.color === 'default')) && {
                        backgroundColor: '#e0e0e0',
                        color: 'rgba(0, 0, 0, 0.87)',
                    }),
                }),
            },
        },
    },
});

const App = () => {
    const [loadingModalOpen, setLoadingModalOpen] = React.useState(false);
    const [messageModalOpen, setMessageModalOpen] = React.useState(false);
    const [messageModalTitle, setMessageModalTitle] = React.useState('');
    const [messageModalMessage, setMessageModalMessage] = React.useState('');

    const dispatch = useDispatch();

    const toggleLoadingModalOpen = () => {
        setLoadingModalOpen(prevLoadingModalOpen => !prevLoadingModalOpen);
    };

    const showMessageModal = (title, message) => {
        setMessageModalTitle(title);
        setMessageModalMessage(message);
        setMessageModalOpen(true);
    };

    const closeMessageModal = () => {
        setMessageModalOpen(false);
    };

    // Single-user, no-auth desktop build: there is no login step to assemble the initial
    // payload, so fetch it once on startup and hydrate the redux store directly.
    React.useEffect(() => {
        (async function loadInitialData() {
            setLoadingModalOpen(true);
            try {
                const data = await initialDataService.getInitialData();
                dispatch({type: 'setAccounts', payload: data.accounts});
                dispatch({type: 'setCategories', payload: data.categories});
                dispatch({type: 'setScheduledTransactions', payload: data.scheduled_transactions});
            } catch (e) {
                showMessageModal('Error', 'An error occurred while loading your data, please restart the app.');
            } finally {
                setLoadingModalOpen(false);
            }
        })();
    }, []);  // eslint-disable-line react-hooks/exhaustive-deps

    return (
        <ThemeProvider theme={theme}>
            <LocalizationProvider dateAdapter={AdapterMoment}>
                <MessageModalContext.Provider value={{showMessageModal, closeMessageModal}}>
                    <LoadingModalContext.Provider value={toggleLoadingModalOpen}>
                        <CssBaseline/>
                        <LoadingModal
                            open={loadingModalOpen}
                        />
                        <MessageModal
                            open={messageModalOpen}
                            title={messageModalTitle}
                            message={messageModalMessage}
                            handleClose={() => setMessageModalOpen(false)}
                        />
                        <Router>
                            <Routes>
                                <Route path='/' element={<AccountList/>}/>
                                <Route path='/settings' element={<Settings/>}/>
                                <Route path='/accounts' element={<AccountList/>}/>
                                <Route path='/accounts/new' element={<NewAccount/>}/>
                                <Route path='/accounts/edit/:id' element={<EditAccount/>}/>
                                <Route path='/categories/new/:type' element={<NewCategory/>}/>
                                <Route path='/categories/edit/:id' element={<EditCategory/>}/>
                                <Route path='/categories/' element={<CategoryList/>}/>
                                <Route path='/categories/:type' element={<CategoryList/>}/>
                                <Route path='/transactions/account/:accountId' element={<TransactionList/>}/>
                                <Route path='/transactions/account/:accountId/new/:type' element={<NewTransaction/>}/>
                                <Route path='/transactions/:transactionId' element={<EditTransaction/>}/>
                                <Route path='/transfers/:transferId/from/:fromAccountId' element={<EditTransfer/>}/>
                                <Route path='/scheduled-transactions' element={<ScheduledTransactionsList/>}/>
                                <Route path='/scheduled-transactions/new/:type' element={<NewScheduledTransaction/>}/>
                                <Route path='/scheduled-transactions/edit/:scheduledTransactionId'
                                       element={<EditScheduledTransaction/>}/>
                                <Route path='/scheduled-transfers/edit/:scheduledTransferId'
                                       element={<EditScheduledTransfer/>}/>
                                <Route path='/scheduled-transactions/pay/:scheduledTransactionId'
                                       element={<PayScheduledTransaction/>}/>
                                <Route path='/scheduled-transfers/pay/:scheduledTransferId'
                                       element={<PayScheduledTransfer/>}/>
                            </Routes>
                            <BottomNavBar/>
                        </Router>
                    </LoadingModalContext.Provider>
                </MessageModalContext.Provider>
            </LocalizationProvider>
        </ThemeProvider>
    );
}

export default App;
