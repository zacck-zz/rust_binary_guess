//import the io library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        //variable to hold the guess
        //note this is a mutable variable as provided by the mut keyword
        let mut guess = String::new();

        //read input from std in, into the guess variable
        io::stdin()
            .read_line(&mut guess)
            //failure handler with the Result type this line will be called if the above fails
            .expect("Failed to read line");

        //convert guess into a real number using shadowing to reuse the previous variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        //compare the guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
