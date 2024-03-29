use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    //inclusivo para baixo mas exclusivo para cima, por isso 101
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        if guess < 1 || guess > 100 {
            println!("The secret number will br between 1 and 100");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {println!("You win");
                                break;},
        }
    }
}
