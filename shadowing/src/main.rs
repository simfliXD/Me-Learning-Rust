// Shadowing is not the same as marking a variable as mut 
fn main() {
	// Shadowing over x. Compiler only see the last x
    let x = 5;

    let x = x + 1; // A new variable x is = the old variable x + 1

    let x = x * 1000;

    {
    	let x = x * 2;
     println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in main function is: {x}");

    /*
     THIS IS A BLOCK COMMENT
     */
}
