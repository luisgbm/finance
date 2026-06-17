import {Card, CardHeader, IconButton} from "@mui/material";
import {Link} from "react-router-dom";
import CreateIcon from "@mui/icons-material/Create";
import Typography from "@mui/material/Typography";
import React from "react";
import moment from "moment";
import {moneyFormat} from "../../utils/utils";

const TransferCard = (props) => {
    const {transaction, fromAccountId} = props;

    const getValueSx = (categoryType) => {
        if (categoryType === 'TransferExpense') {
            return {color: 'red'};
        } else if (categoryType === 'TransferIncome') {
            return {color: 'green'};
        }
    };

    const getTransferCaption = (transaction) => {
        if (transaction.category_type === 'TransferExpense') {
            return `Transfer to ${transaction.account_name}`;
        } else {
            return `Transfer from ${transaction.from_account_name}`;
        }
    };

    return (
        <Card variant='outlined' sx={{mb: 3}}>
            <CardHeader
                action={
                    <IconButton
                        component={Link}
                        to={`/transfers/${transaction.id}/from/${fromAccountId}`}
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
                        <b>{getTransferCaption(transaction)}</b>
                        <br/>
                        <b>Date:</b> {moment(transaction.date).format('DD/MM/YYYY HH:mm')}
                    </>
                }
            />
        </Card>
    );
};

export default TransferCard;
