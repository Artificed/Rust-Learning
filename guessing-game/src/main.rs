use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let rand_num = rand::rng().random_range(1..=100);
    println!("Random Number: {}", rand_num);

    loop {
        let mut guess_num = String::new();

        print!("Enter Guess: ");
        io::stdout().flush().expect("Failed to output!");

        io::stdin()
            .read_line(&mut guess_num)
            .expect("Failed to get input!");

        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(guess_num) => guess_num,
            Err(_) => continue,
        };

        match guess_num.cmp(&rand_num) {
            Ordering::Less => println!("Guessed Less!"),
            Ordering::Greater => println!("Guessed More!"),
            Ordering::Equal => {
                println!("Guessed Correctly!");
                return;
            }
        }
    }
}
