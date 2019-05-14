use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();

    println!("Type your guess!");
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read input line");
    let guess: u32 = guess.trim().parse()
        .expect("Please, type a number"); 

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
    }

    println!("The secret number: {}", secret_number);
    println!("You guessed {}", guess);

    
}