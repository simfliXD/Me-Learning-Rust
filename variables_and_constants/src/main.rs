fn main() {
    variables();
    constants();
}

fn variables() {
	// imutable
    let _a = 5;// "_" means the type is unspecified
    println!("A unspecified value of _a is {}", _a);

    // mutable
    let mut b: i16 = 10;
    println!("Value of b is: {}", b);
    b = 15;
    println!("Value of b is now: {}", b);
}


// Constants are always UPPER case
fn constants() {
    const B: u8 = 2;
    println!("Value of constant b is: {}", B);

    println!("Value of constant PI is: {}", PI);

    println!("Value of 3 Hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}

// You can declare constants outside the global scope "main()"
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;