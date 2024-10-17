use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is: {secret_number}");

    println!("Input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // references are immutable by default, defining mut
        .expect("Failed to read the line");

    let guess: u32 = guess.trim().parse().expect("Please type a number..");

    println!("You guess: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
