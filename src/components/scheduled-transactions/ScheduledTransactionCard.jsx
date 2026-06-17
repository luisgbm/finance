import {Badge, Card, CardHeader, IconButton, ListItemIcon} from "@mui/material";
import CreateIcon from "@mui/icons-material/Create";
import Typography from "@mui/material/Typography";
import {moneyFormat} from "../../utils/utils";
import React from "react";
import Menu from '@mui/material/Menu';
import MenuItem from '@mui/material/MenuItem';
import MoreVertIcon from '@mui/icons-material/MoreVert';
import AttachMoneyIcon from '@mui/icons-material/AttachMoney';
import {Link} from "react-router-dom";
import moment from "moment";

const ScheduledTransactionCard = (props) => {
    const [anchorEl, setAnchorEl] = React.useState(null);
    const open = Boolean(anchorEl);

    const handleClick = (event) => {
        setAnchorEl(event.currentTarget);
    };

    const handleClose = () => {
        setAnchorEl(null);
    };

    const {scheduledTransaction} = props;

    const getValueSx = (categoryType) => {
        if (categoryType === 'Expense') {
            return {color: 'red'};
        } else if (categoryType === 'Income') {
            return {color: 'green'};
        }
    };

    const transactionIsDue = (date) => {
        let dateMoment = moment(date);
        let today = moment();

        return today.isSameOrBefore(dateMoment);
    };

    return (
        <Card key={scheduledTransaction.id} variant='outlined' sx={{mb: 3}}>
            <CardHeader
                action={
                    <>
                        <IconButton onClick={handleClick}>
                            <MoreVertIcon/>
                        </IconButton>
                        <Menu
                            anchorEl={anchorEl}
                            keepMounted
                            open={open}
                            onClose={handleClose}
                        >
                            <MenuItem component={Link}
                                      to={scheduledTransaction.kind === 'Transaction' ? `/scheduled-transactions/edit/${scheduledTransaction.id}` : `/scheduled-transfers/edit/${scheduledTransaction.id}`}>
                                <ListItemIcon>
                                    <CreateIcon/>
                                </ListItemIcon>
                                Edit
                            </MenuItem>
                            <MenuItem component={Link}
                                      to={scheduledTransaction.kind === 'Transaction' ? `/scheduled-transactions/pay/${scheduledTransaction.id}` : `/scheduled-transfers/pay/${scheduledTransaction.id}`}>
                                <ListItemIcon>
                                    <AttachMoneyIcon/>
                                </ListItemIcon>
                                Pay
                            </MenuItem>
                        </Menu>
                    </>
                }
                title={
                    <Badge variant="dot" color="secondary" invisible={transactionIsDue(scheduledTransaction.next_date)}>
                        <Typography
                            variant='h6'
                            sx={getValueSx(scheduledTransaction.category_type)}
                        >
                            {moneyFormat(scheduledTransaction.value)}
                        </Typography>
                    </Badge>
                }
                subheader={
                    <>
                        {
                            scheduledTransaction.description !== '' ? <>
                                <b>Description:</b> {scheduledTransaction.description}<br/></> : <></>
                        }
                        {
                            scheduledTransaction.kind === 'Transaction' ? <>
                                <b>Account:</b> {scheduledTransaction.account_name}<br/></> : <></>
                        }
                        {
                            scheduledTransaction.kind === 'Transaction' ? <>
                                <b>Category:</b> {scheduledTransaction.category_name} ({scheduledTransaction.category_type})<br/></> : <></>
                        }
                        {
                            scheduledTransaction.kind === 'Transfer' ? <>
                                <b>To:</b> {scheduledTransaction.destination_account_name}<br/></> : <></>
                        }
                        {
                            scheduledTransaction.kind === 'Transfer' ? <>
                                <b>From:</b> {scheduledTransaction.origin_account_name}<br/></> : <></>
                        }
                        {
                            scheduledTransaction.repeat === true ? <>
                                <em>Payment {scheduledTransaction.current_repeat_count + 1} of {scheduledTransaction.infinite_repeat ? '(infinite)' : scheduledTransaction.end_after_repeats}</em></> : <></>
                        }
                    </>
                }
            />
        </Card>
    );
};

export default ScheduledTransactionCard;