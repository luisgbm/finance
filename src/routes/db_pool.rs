use rocket_contrib::database;
use rocket_contrib::databases::diesel;

#[database("finance_db")]
pub struct FinancePgDatabase(diesel::PgConnection);
