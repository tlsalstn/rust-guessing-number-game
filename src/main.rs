use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("---- Guessing Game ----");
    println!("Enter the number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Unexpect error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter type of number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Up"),
            Ordering::Greater => println!("Down"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
        }
    }
}
