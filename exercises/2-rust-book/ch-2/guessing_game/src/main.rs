use std::{ io, cmp::{ Ordering } };
use rand::{ Rng };


fn main() {
    println!("Guess the number!");

    let mut secret_number = rand::thread_rng().gen_range(1..=100); // start..=end

    loop {

        println!("Please input your guess");
        
        let mut guess = String::new(); // a function that returns a new instance of a String

        io::stdin()
            .read_line(&mut guess) // The & indicates that this argument is a reference
            .expect("Failed to read line");

        // Shadowing 
        let guess: u32 = match guess.trim().parse() { // The parse method on strings converts a string to another type
            Ok(num) => num,
            Err(_) => continue,
        };


        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
