use rand::Rng;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io;

// Secret number range
const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 100;

const MAX_ATTEMPTS: u32 = 10;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(MIN_NUMBER..=MAX_NUMBER);

    let mut attempts: u32 = 0;

    let mut input: String = String::new();
    let mut guessed_numbers: BTreeSet<u32> = BTreeSet::new();

    println!("Guess the number!");

    loop {
        input.clear();

        if attempts >= MAX_ATTEMPTS {
            println!(
                "You lose, because you reached max attempts ({})",
                MAX_ATTEMPTS
            );
            println!("The secret number was {secret_number}!");
            break;
        } else {
            println!("You have {} attempts left.", MAX_ATTEMPTS - attempts);
        }

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read guess.");

        // Trim whitespace and remove "\n" from enter press then parse to unsigned 32bit intiger, if err prompt to type an intiger.
        let guess: u32 = match input.trim().parse::<u32>() {
            // No error and is within bounds
            Ok(num) if (MIN_NUMBER..=MAX_NUMBER).contains(&num) => num,
            //
            Ok(num) => {
                println!(
                    "Guess is out of bounds: {} (must be between {} and {})",
                    num, MIN_NUMBER, MAX_NUMBER
                );
                continue;
            }
            Err(_) => {
                println!("Please type a valid integer.");
                continue;
            }
        };

        if guessed_numbers.contains(&guess) {
            println!("You've already guessed that number\n");
            continue;
        }
        
        attempts += 1; // We now have an valid non duplicate guess so add 1
        println!("\nYour guess is: {guess}");

        // Push the guess to the vector and sort it in numeric order
        guessed_numbers.insert(guess);

        // compare the secret number to the guess and print result.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You won, in {attempts} attempts!");
                break;
            }
        };

        println!("Your guesses are: {:?}\n", guessed_numbers);
    }
}
