import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import {Container, IconButton} from "@mui/material";
import {Link} from "react-router-dom";
import {Add} from "@mui/icons-material";
import React from "react";
import ScheduledTransactionCard from "./ScheduledTransactionCard";
import moment from "moment";
import {useSelector} from "react-redux";

const ScheduledTransactionsList = () => {
    const scheduledTransactions = useSelector(state => {
        let grouped = {};

        let orderedByDate = [...state.scheduledTransactions].sort((a, b) => moment(a.next_date).diff(moment(b.next_date)));

        for (let t of orderedByDate) {
            let nextDate = moment(t.next_date).format("DD/MM/yyyy");

            if (!grouped[nextDate]) {
                grouped[nextDate] = [];
            }

            grouped[nextDate].push(t);
        }

        return grouped;
    });

    const dateIsToday = (date) => {
        let today = moment().format("DD/MM/yyyy");

        return today === date;
    };

    return (
        <>
            <AppBar position='sticky'>
                <Toolbar>
                    <Typography variant='h6' sx={{flexGrow: 1}}>Scheduled Transactions</Typography>
                    <IconButton color='inherit' component={Link} to={'/scheduled-transactions/new/transaction'}>
                        <Add/>
                    </IconButton>
                </Toolbar>
            </AppBar>
            <Container maxWidth='sm' sx={{p: 3}}>
                {
                    Object.keys(scheduledTransactions).map(date =>
                        <div key={date}>
                            <Typography key={date} variant='h6'
                                        sx={{textAlign: 'center', color: 'grey', mb: 1}}>{dateIsToday(date) ? 'Today' : date}</Typography>
                            {
                                scheduledTransactions[date].map(scheduledTransaction =>
                                    <ScheduledTransactionCard
                                        key={scheduledTransaction.id}
                                        scheduledTransaction={scheduledTransaction}
                                    />
                                )
                            }
                        </div>
                    )
                }
            </Container>
        </>
    );
};

export default ScheduledTransactionsList;