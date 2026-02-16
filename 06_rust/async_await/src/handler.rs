// use std::fs;
// use crate::model::Student;


// /// get all students from the file and store them in the vector 
// pub fn load_employee() -> Vec<Student> {
//     fs::read_to_string("students.json")
//         .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
//         .unwrap_or_else(|_| vec![])
// }


use tokio::fs;

use crate::model::Employee;


pub async fn load_employee() -> Result<Vec<Employee>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("employee.json").await.unwrap();
    let employees = serde_json::from_str(&data)?;
    Ok(employees)
}


pub async fn save_data(employees: &[Employee]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(employees)?;
    fs::write("employee.json", json).await?;
    Ok(())
}