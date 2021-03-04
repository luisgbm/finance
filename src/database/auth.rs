use std::env;
use std::str::FromStr;

use diesel::prelude::*;

use crate::database::models::{AppUser, NewAppUser};

pub fn new_user(new_user: &NewAppUser, connection: &PgConnection) -> QueryResult<AppUser> {
    use crate::database::schema::app_users;
    use diesel::sql_types::{Integer, Text};

    sql_function!(fn gen_salt(salt_type: Text, iter: Integer) -> Text);
    sql_function!(fn crypt(password: Text, salt: Text) -> Text);

    let bf_rounds = env::var("BF_ROUNDS")
        .expect("BF_ROUNDS must be set");

    let bf_rounds = i32::from_str(bf_rounds.as_str())
        .expect("BF_ROUNDS must be numeric");

    diesel::insert_into(app_users::table)
        .values((
            app_users::name.eq(new_user.name.clone()),
            app_users::password.eq(crypt(new_user.password.clone(), gen_salt("bf", bf_rounds)))
        ))
        .get_result(connection)
}

pub fn get_user_by_name(user_name: &str, connection: &PgConnection) -> QueryResult<AppUser> {
    use crate::database::schema::app_users::dsl::*;

    app_users
        .filter(name.eq(user_name))
        .first::<AppUser>(connection)
}

pub fn authenticate_user(user: &NewAppUser, connection: &PgConnection) -> QueryResult<AppUser> {
    use crate::database::schema::app_users::dsl::*;
    use diesel::sql_types::Text;

    sql_function!(fn crypt(provided_password: Text, password_in_db: Text) -> Text);

    let user_in_db = get_user_by_name(&user.name, connection);

    match user_in_db {
        Ok(user_in_db) => {
            app_users
                .filter(name.eq(user.name))
                .filter(password.eq(crypt(user.password, user_in_db.password)))
                .first::<AppUser>(connection)
        }
        Err(err) => Err(err)
    }
}