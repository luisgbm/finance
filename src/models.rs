use serde::{Serialize, Deserialize};

use crate::schema::categorytypes;

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