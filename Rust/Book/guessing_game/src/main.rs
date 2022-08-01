use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;


fn flush(){
    io::stdout().flush().unwrap();
}

fn get_number() -> u32 {


    loop {
        let mut guess_inp = String::new();

        print!("Please input your guess: ");flush();

        io::stdin()
            .read_line(&mut guess_inp)
            .expect("Failed to read line");
        
        match guess_inp.trim().parse() {
            Ok(num) => {
                return num;
            }
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
    }

}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess;

    loop {

        guess = get_number();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}




