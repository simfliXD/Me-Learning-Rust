use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_ATTEMPTS: u32 = 10;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut attempts: u32 = 0;

    let mut guess: String = String::new();
    let mut guessed_numbers: Vec<u32> = Vec::new();

    println!("Guess the number!");

    loop {
        guess.clear();

        println!("You have {} attempts left.", MAX_ATTEMPTS - attempts);
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        println!("Your guess is: {guess}");

        // Trim whitespace and remove "\n" from enter press then parse to unsigned intiger, if err prompt to type an intiger.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Return the formated guess
            Err(_) => {
                println!("Please type an positive integer!");
                continue;
            }
        };

        attempts += 1;
        if attempts >= MAX_ATTEMPTS {
            println!(
                "You lose, because you reached max attempts {}",
                MAX_ATTEMPTS
            );
            break;
        }

        guessed_numbers.push(guess);
        guessed_numbers.sort();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won in {attempts} attempts!");
                break;
            }
        };

        println!("Your guesses are: {:?}", guessed_numbers);
    }
}
