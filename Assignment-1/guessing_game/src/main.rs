use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please Enter the Number:");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to take input");
        
        let guess: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You Guessed: {num}");

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
