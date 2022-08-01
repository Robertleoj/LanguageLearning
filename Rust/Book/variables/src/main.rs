use std::io::{self, Write};
use rand::Rng;

fn get_str_input(prompt: &String) -> String {
    let mut inp = String::new();

    print!("{prompt}: ");io::stdout().flush().expect("wtf");

    io::stdin()
        .read_line(&mut inp)
        .expect("Bruh");

    return inp;
}


fn get_idx_input(prompt: &String)->usize{

    loop {

        let inp = get_str_input(&prompt);

        match inp.trim().parse::<usize>() {
            Ok(n) => {
                return n;
            },
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
    }

}

fn main() {
    let mut x = [0;20];

    for i in 0..x.len(){
        let rand_n: i32 = rand::thread_rng().gen_range(0..100);
        x[i] = rand_n;
    }

    loop{
        
        let idx = get_idx_input(&"Gimme idx".to_string());

        let el = x[idx];

        println!("Value is {el}");
    }
}
