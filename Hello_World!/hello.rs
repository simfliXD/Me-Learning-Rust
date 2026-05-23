fn main() {
	hello_world();
	tell_height(183);
	human_id("Simon", 17, 183.10111);

	let _x = {
		let price = 5;
		let quantity = 10;
		price * quantity
	};
	println!("Result is: {}", _x);

	// addition(542,58)
	let y = addition(542,58);
	println!("542 + 58 is: {}", y);
	println!("Value from function 'addition' is {}", addition(100,150));

	// BMI function call
	let vikt = 72.2;
	let längd = 1.82;
	println!("BMI is: {:.2}", beräkna_bmi(vikt, längd));
}

fn hello_world(){
	println!("Hello, Rust 🦀!");
}

fn tell_height(height: u16){
	println!("My height is {} cm.", height)
}

// Sätt in flera värden
fn human_id(name: &str, age: u32, height: f32){
	println!("My name is {}, I'm {} years old, and my height is {} cm.", name, age, height)
}

// Exression och Statements
// 
// Expression: (Returns a value)
// 5
// true/false
// add(3,5)
// if {code}; else {code};
// 
// funtion returning value
fn addition(a:i32, b: i32) -> i32{
	a + b
}

// Statement: (Does not return a value)
// Variablar let x = 5;
// definiton af funktioner fn main(){}
// if, else, while, etc.

// BMI BERÄKNING
// BMI = vikt(kg)/längd(m)^2
fn beräkna_bmi(vikt_kg: f64, längd_m: f64) -> f64{
	vikt_kg / (längd_m * längd_m)
}