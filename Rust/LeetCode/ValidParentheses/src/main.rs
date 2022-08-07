struct Solution;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

macro_rules! str {
    ($($x:expr), *) => ($($x.to_string()), *);
}

enum Bracc {
    Open(BraccType),
    Close(BraccType)
}

enum BraccType {
    Curly,
    Brace,
    Square
}

use crate::BraccType::{Curly, Brace, Square};
use crate::Bracc::{Open, Close};

impl Bracc {
    fn to_bracc(c: char) -> Bracc {
        match c {
            '{' => Open(Curly),
            '}' => Close(Curly),
            '(' => Open(Brace),
            ')' => Close(Brace),
            '[' => Open(Square),
            ']' => Close(Square),
            _ => panic!("Not a bracc")
        }
    }

}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stacc = Vec::new();

        for c in s.chars() {
            let b = Bracc::to_bracc(c);
            match &b {
                Open(a) => {
                    stacc.push(b);
                },
                Close(a) => {
                    match stacc.pop(){
                        Some(r) => {
                            match (r, a)  {
                                (Open(Curly), Curly) => {},
                                (Open(Brace), Brace) => {},
                                (Open(Square), Square) => {},
                                _ => {return false;}
                            }
                        },
                        None => {return false;}
                    }
                }
            }
        }

        stacc.len() == 0

    }
}

#[test] 
fn test1(){
    let s= str!("()");
    assert!(Solution::is_valid(s));
}

#[test] 
fn test2(){
    let s= str!("()[]{}");
    assert!(Solution::is_valid(s));
}

#[test] 
fn test3(){
    let s= str!("(]");
    assert!(!Solution::is_valid(s));
}


fn main() {
    println!("Hello world!");
}
