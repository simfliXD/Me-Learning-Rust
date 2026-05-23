fn main() {
    let age: u16 = 18;
    if age >= 18 {
    	println!("You can drive a car!");
    } else {
    	println!("You can't drive a car!");
    }

    let number = 6;

    if number % 4 == 0 {
    	println!("The number {} is divisible by 4!", number);
    } else if number % 3 == 0 {
    	println!("The number {} is divisible by 3!", number);
    } else {
    println!("{number} is not divisible by neither 4 or 3!")
    }

    // if condition inside a let statement
    let condition:bool = true;
    let number2 = if condition  {5} else {10};
    println!("Number is: {number2}")
}
