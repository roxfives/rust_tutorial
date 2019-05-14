use std::io;

fn main() {
    let mut guess = String::new();

    println!("Type your guess!");
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read input line");

    println!("You guessed {}", guess);
}