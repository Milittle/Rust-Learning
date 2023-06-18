use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing number!");
    let secret_number = rand::thread_rng().gen_range(0..101);
    loop {
        println!("Please input your guess: ");
        let mut guess = String::from("");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too less"),
            Ordering::Greater => println!("Too bigger"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("You guessed: {}", guess);
    }
}
