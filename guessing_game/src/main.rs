// bring io library into scope (io library comes from standard library std)
use std::io;
// the default libraries brought into scope are included in "The Prelude"
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng, creates generator local to thread of execution and OS
    // gen_range, generates number between 1 includsive and 101 exclusive
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");

        // let creates a variable (e.g. let foo = bar, creates variable foo and binds to value of bar variable)
        // variables by default are IMMUTABLE, need to use mut before variable name to make variable mutable
        // String:new is function that returns new instance of String
            // provided by std and is a growable, UTF-8 encoded text
            // :: syntax indicates that new is an associated function of String type
                // associated function is implemented on a type rather than on an instance
                // known as static methods in some other languages
        let mut guess = String::new();

        // calling stdin function from io module
            // if we had not written use std::io; at start of code, we could've written this as std::io::stdin
            // returns instance of std::io::Stdin, handle to standard input of terminal
        // .read_line calls read_line method on standard input handle to get input from user
            // takes string as argument
                // string must be mutable so the method can change the string's content
            // returns io::Result, Rust also has Result types for each library (i.e. std::Result)
                // Results are enumerations (fixed set of values, variants) (in this case, Ok or Err)
        // & indicates that this argument is a reference
            // references are immutable by default, hence the &mut
        // separate method calls, two lines for two method calls
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
            // read_line returns io::Result, Rust also has Result types for each library (i.e. std::Result)
                // Results are enumerations (fixed set of values, variants) (in this case, Ok or Err)
                // Err will result in error text contained in .expect()
                // Ok will return number of bytes that user entered in read_line
            // not using expect() will result in warning, stating that program hasn't handled possible error

        // SHADOW old value of guess w new one
        // trim whitespace
        // parse method on string parses into a number
            // using : after let guess: means we will annontate the type ourselves
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is catchall value, match all Err values
            Err(_) => continue,
        };
        // moving from using .expect to display crash message to match to handle exception
            //.expect("Please type a number!");

        // {} used to print values
        println!("You guessed: {}", guess);

        // cmp method compares two values and returns variant of Ordering enumeration
        // use match to determine which arm of code to execute based on variant returned
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
