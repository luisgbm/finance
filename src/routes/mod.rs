use rocket::Route;

pub mod models;
pub mod accounts;
pub mod auth;
pub mod categories;
pub mod scheduled_transactions;
pub mod transactions;
pub mod transfers;
pub mod auth_guard;

pub fn get_all_routes() -> Vec<Route> {
    let mut all_routes = Vec::new();

    all_routes.append(&mut categories::get_routes());
    all_routes.append(&mut accounts::get_routes());
    all_routes.append(&mut auth::get_routes());
    all_routes.append(&mut transactions::get_routes());
    all_routes.append(&mut transfers::get_routes());
    all_routes.append(&mut scheduled_transactions::get_routes());

    all_routes
}
