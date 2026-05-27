fn main(){
	ownership();
	transfered_ownership();
	out_of_scope();
	references();
	stuct_example();
}

// Ownership
// Varje värde har EN ägare
// Det kan bara vara EN ägare åt gången
// När ägaren går utnför räckvid, värdet blir släppt

fn ownership(){
	let s1 = String::from("RUST");	
	let len = calculate_lenth(&s1); // & betyder att det är en referens till "s1"
	println!("The length of '{}' is {}", s1, len);
}

fn calculate_lenth(s: &String) -> usize{
	s.len()
}

fn transfered_ownership(){
	let s1 = String::from("RUST");	
	let s2 = s1;
	//println!("s1 {}", s1)
	println!("s2 {}", s2)
}

fn out_of_scope(){
	let s1 = String::from("RUST");
	let len = calculate_lenth(&s1);
	println!("The length of '{}' is {}", s1, len);
//	print_s1();
}// s1 går utanför räckvid och blir släppt

//fn print_s1(){
//	println!("s {}", s1);
//}


// Referenser
// kan vara ändringsbara och ej ändringsbara
// "&" som suffix
fn references (){
	let mut _x: i32 = 5;
	let _r: &mut i32 = &mut _x;

	*_r += 1;
	*_r -= 3;
	
	println!("_r = {}", _r);
}

// Struct
fn stuct_example(){
	let mut account:BankAccount = BankAccount { 
		owner: "Simon".to_string(),
		balance: 1019.2,
	};
	// Immutable borrow to check balance
	account.check_balance();

	// Mutable borrow
	account.withdraw(99.99);

	account.check_balance();
}

// structs are mutable by default
struct BankAccount {
	owner: String,
	balance:f64,
}

impl BankAccount {
	fn withdraw(&mut self, amount: f64){
		println!("Withdrawing {} from an account owned bt {}", amount, self.owner);
		self.balance -= amount;
	}
	fn check_balance(&self){
		println!("Account owned by {} has a balance of {}", self.owner, self.balance);
	}
}