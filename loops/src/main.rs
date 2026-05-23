// loop - forever
// while - while condition is true
fn main() {
	//loop{
	//	println!("Hello, world!");
	//}
	let mut counter = 0;

	let result = loop {
		counter +=1;

		if counter == 10 {
			break counter * 2; // counter * 2 then break
		}
	};
	println!("Result of counter is {result}"); // Gives 20

	// Loop Lables
	let mut count = 0;
	'counting_up: loop {
		println!("count = {count}");
		let mut remaining = 10;

		loop {
			println!("remaining = {remaining}");
			if remaining == 9 {
				break;
			}
			if count == 2 {
				break 'counting_up;
			}
			count += 1;
			remaining -= 1;
		}
	}


	let mut number = 3;
	while number != 0 {
		println!("{number}");
		number -= 1;
	}
	println!("GO!");
	

	let array  = [1, 2, 3, 4, 5, 6];
	let letters = ["a", "b", "c", "d"];
	
	for element in array {
		println!("{element}");
	}
	for letter in letters {
		println!("{letter}");
	}
}
