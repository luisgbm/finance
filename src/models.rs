use serde::{Serialize, Deserialize};

use crate::schema::categorytypes;
use crate::schema::categories;

#[derive(Queryable, Serialize, Deserialize)]
pub struct CategoryType {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="categorytypes"]
pub struct NewCategoryType<'a> {
    pub name: &'a str
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub categorytype: i32,
    pub name: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub categorytype: i32,
    pub name: &'a str
}