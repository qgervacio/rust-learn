use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("----------------");
    println!("Guess the number");
    println!("----------------\n");
    
    // able to infer that secret_number is a type of integer
    let secret_number = rand::thread_rng().gen_range(1, 6);

    loop {
        println!("Please input your guess (between 1-5 inclusive)");

        // new, like in Java is a static method.
        // in Rust it is called a associated function.
        // mut means mutable. by default in Rust, variables
        // are immutable.
        let mut guess = String::new();

        // if not for line #1, this would have been std::io::stdin()
        io::stdin() 
            .read_line(&mut guess)
            .expect("Failed to read line");
        print!("You guessed: {}", guess);

        // convert to integer. Rust allow you to "shadow" guess
        // with a new one. this is to allow you to change the type of
        // guess instead of creating a new one perhaps called guess_str
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }, 
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win! The secret number is {}", secret_number);
                break;
            }
        }
    }
}
