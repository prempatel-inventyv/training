struct User{
	username:String,
	password:String,
	fullname:String,
	email:String
}

impl User {

	fn set_email(&mut self,email:String){
		self.email = email
	}

	fn set_password(&mut self,password:String){
		self.password = password
	}

	fn set_fullname(&mut self,fullname:String){
		self.fullname = fullname
	}

	fn get_username(&self) -> String{
		self.username.clone()
	}

	fn set_username(&mut self,username:String){
		self.username = username
	}

	fn get_email(&self) -> String{
		self.email.clone()
	}
	
	fn get_password(&self) -> String {
		self.password.clone()
	}
	
	fn get_fullname(&self) -> String{
		self.fullname.clone()
	}

	fn check_credential(&self,email:String,password:String) -> bool{
		if self.password == password && self.email == email {
			return true;
		}
		false
	}

	fn login_user(&self,email:String,password:String) -> String{
		let is_credantial_valid :bool = self.check_credential(email, password);
		if is_credantial_valid == true {
			format!("User login successful")
		}
		else{
			format!("Invalid Credantial")
		}
	}

	fn user_details(&self) -> String {
		format!(
			"Username : {}\nPassword : {}\nEmail : {}\nFullname : {}",
			self.username,
			self.password,
			self.email,
			self.fullname
		)
	}	

	fn user_details_with_args(&self,username:String,password:String,email:String,fullname:String) -> String{
		format!("Username : {}\nPassword : {}\nEmail {}\nFullname: {}",username,password,email,fullname)
	}
}

fn main(){
	let mut user1:User = User{
		username:String::from("poojan_0671"),
		password:String::from("123456"),
		fullname:String::from("Prem Patel"),
		email:String::from("poojan@gmail.com")
	};

	user1.set_fullname("Prem B. Patel".to_string());
	user1.set_email("prem@gmail.com".to_string());
	user1.set_username("prem_0671".to_string());
	user1.set_password("123456".to_string());
	println!("{}",user1.get_fullname());
	println!("{}",user1.get_email());
	println!("{}",user1.get_password());
	println!("{}",user1.get_username());

	println!("User Data : {}", user1.user_details());
	// println!("User Data : {}", user1.user_details_with_args());
	println!("{}",user1.login_user("prem@gmail.com".to_string(),"123456".to_string()));
}