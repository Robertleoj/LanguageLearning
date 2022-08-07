struct Solution;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

macro_rules! str {
    ($($x:expr), *) => ($($x.to_string()), *);
}

impl Solution {


    pub fn int_to_roman(num: i32) -> String {
        let mut num = String::new();
    }
}


fn main() {
    println!("Hello world!");
}
