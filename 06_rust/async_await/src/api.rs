use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use uuid::Uuid;
use crate::{SharedState, handler::{save_data}, model::Employee};

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

#[derive(Serialize)]
pub struct SimpleResponse {
    message: String,
}


pub async fn check_health() -> Json<SimpleResponse>{
    Json(
      SimpleResponse { message: "Server is running".to_string() }
    )
  }
  
pub async fn get_employees(State(shared_data): State<SharedState>) ->  impl IntoResponse {
    let employees = shared_data.read().await;
    Json(employees.clone()).into_response()
}
  
pub async fn get_employee(State(shared_data): State<SharedState>,Path(id): Path<String>) -> impl IntoResponse {
    let employee =  shared_data.read().await;
    let emp = employee.iter().find(|emp| emp.id == id);
    match emp {
        Some(data) => Json(
            data.clone()
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "Employee not found".to_string(),
            })
        ).into_response()
    }
  }

pub async fn add_employee(State(shared_data): State<SharedState>, Json(mut employee): Json<Employee>)-> Result<impl IntoResponse, StatusCode> {
    employee.id = Uuid::new_v4().to_string();
    let mut data = shared_data.write().await;
    data.push(employee.clone());
    save_data(&data).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_employee(
    Path(id): Path<String>,
    State(shared_data): State<SharedState>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut data = shared_data.write().await;


    data.retain(|emp| emp.id != id);

    save_data(&data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_employee(
    State(shared_data): State<SharedState>, 
    Json(updated_employee): Json<Employee>, 
    Path(id): Path<String>) 
    -> Result<impl IntoResponse, StatusCode> {
    let mut data = shared_data.write().await;
    if let Some(emp) = data.iter_mut().find(|emp| emp.id == id){
        emp.email = updated_employee.email;
        emp.name = updated_employee.name;
        emp.mobile = updated_employee.mobile;
        Ok(
            Json(
                SimpleResponse{
                    message: "User Updated".to_string()
                }
            ).into_response()
        )
    }
    else{
        Err(StatusCode::NOT_FOUND)
    }
}