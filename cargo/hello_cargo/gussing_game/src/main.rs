use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    println!("Please Enter the Number:");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secrete no is: {}", secret_number);
    println!("Input your guess");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Read failed for the line");
    println!("The nuber you guessed {}", guess);
}

fn guess_higher_lower(){
    println!("You guessed {guess} the number is {secret_number}");
}
