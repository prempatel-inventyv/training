#[derive(Debug,Clone)]
struct Address {
    street: String,
    city: String,
    state: String,
    country: String,
}

#[derive(Debug,Clone)]
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
        self.get_all_details()

    }

    fn get_all_details(&self){
        println!("{}",self.user_details(&self))
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

    fn user_details(&self,user: &User) -> String {

		// println!("user 1 : {:p}", &self);
		// println!("user 2  : {:p}", &user);

        format!(
            "User1 : {:p} \n\nUsername : {}\nPassword : {}\nEmail : {}\nFullname : {}\nStreet : {}\nCity : {}\nState : {}\nCountry : {}\n\nUser2 : {:p} \n\nUsername : {}\nPassword : {}\nEmail : {}\nFullname : {}\nStreet : {}\nCity : {}\nState : {}\nCountry : {}",
            self,
            self.username,
            self.password,
            self.email,
            self.fullname,
            self.address.street,
            self.address.city,
            self.address.state,
            self.address.country,

            &user,
			&user.username,
            &user.password,
            &user.email,
            &user.fullname,
            &user.address.street,
            &user.address.city,
            &user.address.state,
            &user.address.country,
        )
    }
}

fn main() {

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

	let user2= &mut user1;

    user2.set_username("prem_0671".to_string());
    user2.set_password("123456".to_string());
    user2.set_fullname("Prem B. Patel".to_string());
    user2.set_email("prem@gmail.com".to_string());

    let new_address = Address {
        street: "Ring Road".to_string(),
        city: "Kadi".to_string(),
        state: "Gujarat".to_string(),
        country: "India".to_string(),
    };

    user2.set_address(new_address);
	
	
	println!("user 2 : {:p}", user2);
	println!("user 1 : {:p}", &user1);


    println!("{}", user1.get_username());
    println!("{}", user1.get_password());
    println!("{}", user1.get_fullname());
    println!("{}", user1.get_email());

    let addr = user1.get_address();
    println!("City from getter: {}", addr.city);

    
}
