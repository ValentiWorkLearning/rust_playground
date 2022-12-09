use std::io;

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    
    loop{
        println!("Please input your guess!");


        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Seems something went wrong. Try again.");
        let guess_int:i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {guess_int}");

        match guess_int.cmp(&secret_number) {
            Ordering::Less=> println!("Too small"),
            Ordering::Equal => {
                println!("Nice");
                break;
            },
            Ordering::Greater => println!("Try less!"),
        }
    }
}
