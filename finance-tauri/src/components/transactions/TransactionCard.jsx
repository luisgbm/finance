import {Card, CardHeader, IconButton} from "@mui/material";
import {Link} from "react-router-dom";
import CreateIcon from "@mui/icons-material/Create";
import Typography from "@mui/material/Typography";
import React from "react";
import moment from "moment";
import {moneyFormat} from "../../utils/utils";

const TransactionCard = (props) => {
    const {transaction} = props;

    const getValueSx = (categoryType) => {
        if (categoryType === 'Expense') {
            return {color: 'red'};
        } else if (categoryType === 'Income') {
            return {color: 'green'};
        }
    };

    return (
        <Card variant='outlined' sx={{mb: 3}}>
            <CardHeader
                action={
                    <IconButton
                        component={Link}
                        to={`/transactions/${transaction.id}`}
                    >
                        <CreateIcon/>
                    </IconButton>
                }
                title={
                    <Typography
                        variant='h6'
                        sx={getValueSx(transaction.category_type)}
                    >
                        {moneyFormat(transaction.value)}
                    </Typography>
                }
                subheader={
                    <>
                        {
                            transaction.description !== '' ? <>
                                <b>Description:</b> {transaction.description}<br/></> : <></>
                        }
                        <b>Category:</b> {transaction.category_name} ({transaction.category_type})
                        <br/>
                        <b>Date:</b> {moment(transaction.date).format('DD/MM/YYYY HH:mm')}
                    </>
                }
            />
        </Card>
    );
};

export default TransactionCard;
