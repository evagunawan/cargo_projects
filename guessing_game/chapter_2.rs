# Programming a Guessing Game

// import library crate
// the input/output library comes from the standard library known as std
// This library is called the prelude
use std::io;


// entry point of the program
// fn indicates that there is a new function
// () indicate that there are no parameteres
// { the start of the function

fn main() {

    // Calling a macro that will print guess the number to the screen
    println!("Guess the number!");

    // Calling a macro that will print plekase input your guess to the screen
    println!("Please input your guess");

    // Creates a variable that stores the user's input 
    // let strats the process of declaring a variable
    // Variables are immutable (cannot be changed) in Rust
    // to allow variables to be mutable, add mut in front of it
    // A function that returns a new instance of a String
    // :: indicates that new is an associated function of the string type
        // associated functions = a function that is implemented on a type
    // this new funtion creates a new, empty string
    let mut guess = String::new(); //line has created a mutable variable that is currently bound to a new, empty instance of a String

    // calling the stdin function from the io module of the std crate
    io::stdin()
        // calls readline method on the user input
        // Passed &mut guess as the argument to readline method
        // needs to be mutabled so the method can change the string's contents
        // & indicates that this argument is a reference
            // allows you to reference that code in memory multiple times without having to copy that data into memory multiple times 
            // references are immutable by default
            .read_line(&mut guess)

        // error handling, if err is returned will print this issue
        .expect("Failed to read line");

    // {} are placeholders 
    println!("You guessed: {}", guess);

}

// This works. Now we need to generate a random number using a new library crate
// Must import the rand number into the dependencies section of the toml file
// Crates are pulled from crates.io
// Cargo lock file is a way to ensure reproducible builds. Allows rust to create this file the first time cargo build is run
// To update a crate, use cargo update

use std::io;

// import crate
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // creating a variable that is a random number between 1-100
    // rand::thread_rng() this calls the thread rng function that gives us a random number generator to use
    // This is local to the current thread of execution and is seeded by the OS
    // Then we can call the gen_range to generate a random variable between 1 and 100
    // gen_range(start..=end) and is inclusive on the lower and upper bounds
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

// using cargo run on this should generate a random number each time it is run. We don't care about the location of the secret number currently
// The next thing we need to do is to call the std crate, the compare (cmp) module, and the Ordering function/enum
// This .rs code will not compile
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // bind new variable to expression 'guess.trim().parse()
    // .trim() method on a string will eliminate any whitespace at the beginning and end
    // .parse() method will only work on characters that can logically be converted into numbers
    
    let mut guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number is {secret_number}")
}