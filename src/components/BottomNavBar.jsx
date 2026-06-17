import React from "react";
import {Link, useLocation} from "react-router-dom";
import BottomNavigation from "@mui/material/BottomNavigation";
import BottomNavigationAction from "@mui/material/BottomNavigationAction";
import HomeIcon from "@mui/icons-material/Home";
import ImportExportIcon from "@mui/icons-material/ImportExport";
import SettingsIcon from "@mui/icons-material/Settings";
import EventIcon from "@mui/icons-material/Event";
import {Badge} from "@mui/material";
import {dueScheduledTransactions} from "./scheduled-transactions/due.scheduled.transactions";
import {useSelector} from "react-redux";

const BottomNavBar = () => {
    const [value, setValue] = React.useState('home');
    const [hide, setHide] = React.useState(false);

    const allScheduledTransactions = useSelector(state => state.scheduledTransactions);

    let location = useLocation();

    const dueBadgeCount = dueScheduledTransactions(allScheduledTransactions);

    React.useEffect(() => {
        const hideForPaths = ['/', '/users/new'];

        if (hideForPaths.includes(location.pathname)) {
            setHide(true);
        } else {
            setHide(false);
        }

        if (location.pathname.startsWith('/categories')) {
            setValue('categories');
        } else if (location.pathname.startsWith('/settings')) {
            setValue('settings');
        } else if (location.pathname.startsWith('/accounts')) {
            setValue('home');
        } else if (location.pathname.startsWith('/transactions')) {
            setValue('home');
        } else if (location.pathname.startsWith('/transfers')) {
            setValue('home');
        } else if (location.pathname.startsWith('/scheduled-transactions')) {
            setValue('scheduled-transactions');
        }
    }, [location]);

    if (!hide) {
        return (
            <BottomNavigation
                value={value}
                onChange={(event, newValue) => {
                    setValue(newValue);
                }}
                showLabels
                sx={{zIndex: theme => theme.zIndex.drawer + 1, width: '100%', position: 'fixed', bottom: 0}}
            >
                <BottomNavigationAction
                    label='Home'
                    icon={<HomeIcon/>}
                    component={Link}
                    to={'/accounts'}
                    value={'home'}
                />
                <BottomNavigationAction
                    label='Schedule'
                    icon={
                        <Badge badgeContent={dueBadgeCount} color="secondary" invisible={dueBadgeCount === 0}>
                            <EventIcon/>
                        </Badge>
                    }
                    component={Link}
                    to={'/scheduled-transactions'}
                    value={'scheduled-transactions'}
                />
                <BottomNavigationAction
                    label='Categories'
                    icon={<ImportExportIcon/>}
                    component={Link}
                    to={'/categories'}
                    value={'categories'}
                />
                <BottomNavigationAction
                    label='Settings'
                    icon={<SettingsIcon/>}
                    component={Link}
                    to={'/settings'}
                    value={'settings'}
                />
            </BottomNavigation>
        );
    } else {
        return (
            <></>
        );
    }
};

export default BottomNavBar;
