use axum::{
    Router,
    routing::{get, post}
};
use crate::{SharedState, api::{
        add_employee, check_health, delete_employee, get_employee, get_employees
    }};


pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(check_health))
        .route("/get_employees", get(get_employees))
        .route("/get_employee/:id", get(get_employee))
        .route("/add_employee", post(add_employee))
        .route("/delete_employee/id", post(delete_employee))
}

