import {Card, CardHeader, IconButton} from "@mui/material";
import {Link} from "react-router-dom";
import CreateIcon from "@mui/icons-material/Create";
import Typography from "@mui/material/Typography";
import React from "react";

const CategoryCard = (props) => {
    return (
        <Card key={props.categoryId} variant='outlined' sx={{mb: 3}}>
            <CardHeader
                action={
                    <IconButton component={Link} to={`/categories/edit/${props.categoryId}`}>
                        <CreateIcon/>
                    </IconButton>
                }
                title={<Typography variant='h6'>{props.categoryName}</Typography>}
            />
        </Card>
    );
};

export default CategoryCard;
