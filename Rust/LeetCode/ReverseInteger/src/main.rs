struct Solution;

use std::num::Wrapping;
impl Solution {

    fn num_digits(x: i32) -> usize {
        let mut num_digits = 0;
        let mut digx = x;
        while digx != 0 {
            digx /= 10;
            num_digits += 1;
        }
        num_digits
    }

    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0
        }

        let mut x = x;
        let sign = x < 0;

        let num_digits = Self::num_digits(x);

        let mut out = Wrapping(0);

        let mut i = 0;

        while x != 0 {
            let digit= x % 10;
            x /= 10;

            if digit == 0 {i+=1;continue;}

            let mult = num_digits - i - 1;
            
            // println!("{digit}");

            for _ in 0..(if sign {-digit} else {digit}) {
                out = out + Wrapping(((10 as i32).pow(mult as u32)) * (if sign {-1} else {1}));

                // println!("{out}");

                if (out < Wrapping(0)) != (sign){
                    return 0;
                }
            }

            i += 1;

        }
        out.0

    }
}
fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(1234567899));
}
