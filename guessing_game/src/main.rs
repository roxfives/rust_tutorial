use std::io;
use std::cmp::Ordering;
use rand::Rng;

const INITIAL_MESSAGE: &str = "##################### Type your guess! #####################";
const FAILED_STDIN_MESSAGE: &str = "Failed to read input line";
const INVALID_INPUT_MESSAGE: &str = "Please, print a valid integer!";
const GUESS_DESCRIPTION: &str = "You guessed ";
const TOO_LOW_MESSAGE: &str = "Too low!";
const TOO_BIG_MESSAGE: &str = "Too big!";
const WINNING_MESSAGE: &str = "##################### You win! #####################";

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("{}", INITIAL_MESSAGE);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect(FAILED_STDIN_MESSAGE);
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", INVALID_INPUT_MESSAGE);
                continue;
            },
        }; 

        println!("{} {}", GUESS_DESCRIPTION, guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", TOO_LOW_MESSAGE),
            Ordering::Greater => println!("{}", TOO_BIG_MESSAGE),
            Ordering::Equal => {
                println!("{}", WINNING_MESSAGE);
                break;
            }
        }
    }    
}