use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize,Deserialize,Debug)]
struct Address {
    street: String,
    city: String,
    state: String,
    country: String,
}

#[derive(Serialize,Deserialize,Debug)]
struct User {
    username: String,
    password: String,
    fullname: String,
    email: String,
    address: Address,
}


fn main(){
    let user = User {
        username: "poojan_0671".to_string(),
        password: "123456".to_string(),
        fullname: "Prem Patel".to_string(),
        email: "poojan@gmail.com".to_string(),

        address: Address {
            street: "Main Road".to_string(),
            city: "Mehsana".to_string(),
            state: "Gujarat".to_string(),
            country: "India".to_string(),
        },
    };

    let json_string = serde_json::to_string(&user).unwrap();

    let deserialized_user:User = serde_json::from_str(&json_string).unwrap();

    println!("{}",json_string);

    println!("Deserialized {:?}",deserialized_user);

    let row_user_data = r#"
        {
            "username": "prem_0671",
            "password": "123456",
            "fullname": "Prem Patel",
            "email": "prem@gmail.com",
            "address": {
                "street": "Ring Road",
                "city": "Mehsana",
                "state": "Gujarat",
                "country": "India"
        }}"#;

    let row_str_deserialize : User = serde_json::from_str(row_user_data).unwrap();

    println!("row data :  {:?}",row_str_deserialize);
}