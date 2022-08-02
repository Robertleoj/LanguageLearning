use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hs = HashSet::new();
        for n in nums {
            if hs.contains(&n){
                hs.remove(&n);
            } else {
                hs.insert(n);
            }
        }

        return hs.into_iter().next().expect("Wtf");
    }
}

fn main() {
    let v = vec![4, 1, 2, 1, 2];

    println!("{}", Solution::single_number(v));
}
