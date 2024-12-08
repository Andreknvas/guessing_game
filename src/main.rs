use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Welcome to my Rust Handbook guessing game!");
   println!();
   
    println!("Guess the number!");
    println!();

    println!("You have 9 attempts");
   println!();
    
    let numero_secreto = rand::thread_rng().gen_range(1..=100);

for tentativa in 1..9 {
    println!("This is your attempt {}/8", tentativa);
    println!("Input your guess");
    
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("The value is unaceppted");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed the number: {}", guess);

    match guess.cmp(&numero_secreto) {
        Ordering::Less => println!("The number is too low!"),
        Ordering::Greater => println!("The number is too high!"),
        Ordering::Equal => { 
            println!("You guessed it! Congrats!");
            break;
            },
           
        }
        if tentativa == 8 {
            println!("You lose!");
        }
    }

}
