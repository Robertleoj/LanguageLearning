struct Solution;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

macro_rules! str {
    ($($x:expr), *) => ($($x.to_string()), *);
}

impl Solution {
    fn value_of(c: char) -> i32{
        match c {
            'I' =>1,
            'V' =>5,
            'X' =>10,
            'L' =>50,
            'C' =>100,
            'D' =>500,
            'M' =>1000,
            _ => panic!("Invalid")
        }
    }

    pub fn roman_to_int(s: String) -> i32 {

        let c= s.as_bytes();
        let mut n = 0;

        for (idx, ch) in c.iter().enumerate() {
            let v = Self::value_of(*ch as char);
            if idx < c.len() - 1 && v < Self::value_of(c[idx+1] as char) {
                n -= v;
            } else {
                n += v;
            }
        }
        n
        
    }
}


fn main() {
    println!("{}", Solution::roman_to_int(str!("III")));
    println!("{}", Solution::roman_to_int(str!("LVIII")));
    println!("{}", Solution::roman_to_int(str!("MCMXCIV")));
}
