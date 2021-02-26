use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub struct FinanceDB {
    pub db_connection: PgConnection
}

impl FinanceDB {
    pub fn new() -> FinanceDB {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        FinanceDB {
            db_connection: PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url))
        }
    }
}