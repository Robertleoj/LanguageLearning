
struct Solution;
impl Solution {

    fn num_len(mut n: i32) -> u32 {
        let mut len = 0;

        while n > 0 {
            n /= 10;
            len += 1;
        }

        len
    }

    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {return false;};
        
        let mut len = Self::num_len(x);

        let mut left;
        let mut right;

        while len > 1 {
            let pow = (10 as i32).pow(len - 1);
            left = x / (10 as i32).pow(len - 1);
            right = x % 10;


            if left != right {
                return false;
            }

            x -= left * pow;
            x /= 10;
            len -= 2;
        }

        true
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(121));
}
