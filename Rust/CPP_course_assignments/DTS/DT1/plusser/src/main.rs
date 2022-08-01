use std::io::{self, Write};

fn pt1(){
    println!("Hello, world!");
}


fn flush(){
    std::io::stdout().flush().expect("Failed to flush");
}

fn get_std_inp(prompt: &str) -> String {
    loop {
        let mut out = String::new();

        print!("{prompt}: ");flush();

        match io::stdin().read_line(&mut out) {
            Ok(_) => {return out},
            Err(_) => {
                println!("Failed to read line!");
                continue;
            }
        };
    }
}


fn get_char_inp(prompt: &str) -> char {
    loop {
        let inp = get_std_inp(prompt);
        match inp.trim().parse::<char>() {
            Ok(c) => {return c;},
            Err(_) => {
                println!("Please input a single character!");
                continue;
            }
        }
    }
}

fn get_num_inp(prompt: &str) -> i32 {
    loop {
        let inp = get_std_inp(prompt);

        match inp.trim().parse::<i32>() {
            Ok(val) => {
                return val;
            },
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        }
    }
}


fn two_number_add(){
    let x = get_num_inp("x");
    let y = get_num_inp("y");
    println!("{} + {} = {}", x, y, x + y);
}

fn difference(){
    let x = get_num_inp("x");
    let y = get_num_inp("y");
    println!("{} + {} = {}", x, y, x - y);
}

fn counter(){
    let n = get_num_inp("How high?");
    for i in 1..=n {
        println!("{i}");
    }
}

fn pt3(){
    let usage = "Options: 
        'p': Add two numbers
        'm': difference of two numbers
        'q': Quit
    ";

    loop {
        let c = get_char_inp("Input choice");
        match c {
            'p' => {
                two_number_add();
            },
            'm' => {
                difference();
            },
            'c' => {
                counter();
            },
            'q' => {
                println!("Bye");
                std::process::exit(0);
            },
            _ => {
                println!("{usage}");
            }

        }
    }
    
}


fn main() {
    pt3();
}


/* 
fn pt2(){

    let x = get_num_inp("First number");
    let y = get_num_inp("Second number");

    println!("{} + {} = {}", x, y, x + y);
}
*/


