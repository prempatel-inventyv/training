#[derive(Clone)]
struct Address {
    street: String,
    city: String,
    state: String,
    country: String,
}

struct User {
    username: String,
    password: String,
    fullname: String,
    email: String,
    address: Address,
}

impl User {
    fn set_username(&mut self, username: String) {
        self.username = username;
    }

    fn set_password(&mut self, password: String) {
        self.password = password;
    }

    fn set_fullname(&mut self, fullname: String) {
        self.fullname = fullname;
    }

    fn set_email(&mut self, email: String) {
        self.email = email;
    }

    fn set_address(&mut self, address: Address) {
        self.address = address;
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_password(&self) -> &str {
        &self.password
    }

    fn get_fullname(&self) -> &str {
        &self.fullname
    }

    fn get_email(&self) -> &str {
        &self.email
    }

    fn get_address(&self) -> Address {
        self.address.clone()
    }

    fn check_credential(&self, email: &str, password: &str) -> bool {
        self.email == email && self.password == password
    }

    fn login_user(&self, email: &str, password: &str) -> String {
        if self.check_credential(email, password) {
            "User login successful".to_string()
        } else {
            "Invalid Credential".to_string()
        }
    }

    fn user_details(&self) -> String {
        format!(
            "Username : {}\nPassword : {}\nEmail : {}\nFullname : {}\nStreet : {}\nCity : {}\nState : {}\nCountry : {}",
            self.username,
            self.password,
            self.email,
            self.fullname,
            self.address.street,
            self.address.city,
            self.address.state,
            self.address.country,
        )
    }
}

pub fn struct_task() {

    let mut user1 = User {
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

    user1.set_username("prem_0671".to_string());
    user1.set_password("123456".to_string());
    user1.set_fullname("Prem B. Patel".to_string());
    user1.set_email("prem@gmail.com".to_string());

    let new_address = Address {
        street: "Ring Road".to_string(),
        city: "Kadi".to_string(),
        state: "Gujarat".to_string(),
        country: "India".to_string(),
    };

    user1.set_address(new_address);

    println!("{}", user1.get_username());
    println!("{}", user1.get_password());
    println!("{}", user1.get_fullname());
    println!("{}", user1.get_email());

    let addr = user1.get_address();
    println!("City from getter: {}", addr.city);

    println!("\nUser Data:\n{}", user1.user_details());

    println!(
        "\n{}",
        user1.login_user("prem@gmail.com", "123456")
    );
}
