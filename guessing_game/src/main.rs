use rand::Rng;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io;

// Secret number range
const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 100;

const MAX_ATTEMPTS: u32 = 10;

fn main() {
    /*
        enum Difficulty {
            Easy,
            Medium,
            Hard,
        }
    */

    println!("Guess the number!");

    loop {
        play_game();

        println!("Play again? (y/n)");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    
        if input.trim() != "y" && input.trim() != "yes" {
            break;
        }
    }
}

fn play_game() {
	let secret_number: u32 = rand::thread_rng().gen_range(MIN_NUMBER..=MAX_NUMBER);

    let mut attempts: u32 = 0;

    let mut guessed_numbers: BTreeSet<u32> = BTreeSet::new();

    loop {
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

        let guess: u32 = guess_input();

        if !(MIN_NUMBER..=MAX_NUMBER).contains(&guess) {
        	println!("Please enter a guess between {MIN_NUMBER} and {MAX_NUMBER}.\n");
         	continue;
        }
            
        if guessed_numbers.contains(&guess) {
            println!("You've already guessed that number.\n");
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
        let difference: u32 = guess.abs_diff(secret_number);

        if difference <= 5 {
            println!("{guess} is very close!")
        } else if difference <= 15 {
            println!("{guess} is quite close!")
        }

        println!("Your guesses are: {}\n", guessed_numbers.iter().map(u32::to_string).collect::<Vec<String>>().join(", "));
    }
}

fn guess_input() -> u32 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read guess");
        
        // Trim whitespace and remove "\n" from enter press then parse to unsigned 32bit intiger
        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(e) => println!("Error: {e}\nPlease enter a valid integer."),
        }
    }
}
