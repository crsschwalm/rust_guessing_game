use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too large..."),
            Ordering::Equal => {
                println!("Just Right! {} = {}", &secret_number, &guess);
                break;
            }
        }
    }
}
