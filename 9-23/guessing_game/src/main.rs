use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");

    let pw = rand::thread_rng().gen_range(1, 101);

    println!("PW is {}", pw);

    loop {
        println!("Please input the number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&pw) {
            Ordering::Less => println!("Small"),
            Ordering::Equal => {
                println!("Yup");
                break;
            }
            Ordering::Greater => println!("Big"),
        }
    }
}
