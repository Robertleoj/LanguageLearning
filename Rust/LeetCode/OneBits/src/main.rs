struct Solution;

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut n = n;

        let mut num: i32 = 0;
        while n > 0 {
            num += (n & 1) as i32;
            n >>= 1;
        }

        return num;
    }
}

fn main() {
    println!("{}", Solution::hammingWeight(0x0011011));
}
