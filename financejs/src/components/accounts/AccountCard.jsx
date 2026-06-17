import React from 'react';
import {Card, CardHeader, IconButton} from "@mui/material";
import {Link} from "react-router-dom";
import CreateIcon from "@mui/icons-material/Create";
import Typography from "@mui/material/Typography";
import {moneyFormat} from "../../utils/utils";

const AccountCard = (props) => {
    return (
        <Card key={props.accountId} variant='outlined' sx={{mb: 3}}>
            <CardHeader
                action={
                    <IconButton component={Link} to={`/accounts/edit/${props.accountId}`}>
                        <CreateIcon/>
                    </IconButton>
                }
                title={
                    <Link
                        underline='none'
                        to={`/transactions/account/${props.accountId}`}>
                        <Typography variant='h6' sx={{textDecoration: 'none'}}>
                            {props.accountName}
                        </Typography>
                    </Link>
                }
                subheader={
                    <>
                        Balance: <span style={{color: props.accountBalance >= 0 ? 'green' : 'red'}}>
                            {moneyFormat(props.accountBalance)}
                        </span>
                    </>
                }
            />
        </Card>
    );
};

export default AccountCard;
