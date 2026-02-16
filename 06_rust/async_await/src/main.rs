use std::{net::SocketAddr, sync::{Arc}};
use tokio::{net::TcpListener, sync::RwLock};
use axum::{Router};

use crate::{handler::load_employee, model::Employee, routes::routes};

pub mod handler;
pub mod routes;
pub mod model;
pub mod api;

type SharedState = Arc<RwLock<Vec<Employee>>>;

#[tokio::main(flavor = "multi_thread",worker_threads = 5)]
async fn main(){

    let employee = load_employee().await.unwrap();

    let shared_data = Arc::new(RwLock::new(employee));

    let app = Router::new()
        .merge(routes())
        .with_state(shared_data);

    let addr = SocketAddr::from(([0,0,0,0],8000));

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
    .await
    .unwrap();
}