struct Solution;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

macro_rules! str {
    ($($x:expr), *) => ($($x.to_string()), *);
}

use std::collections::BTreeSet;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let btree: BTreeSet<i32> = nums.into_iter().collect();

        let mut ret = Vec::new();

        
        while let Some(first) = first_iter.next(){
            if *first < 0 {break;}

            for val in first_iter.clone() {
                let val3 = -(first + val);
                if btree.contains(&val3){
                    ret.push(vec![*first, *val, val3])
                }
            }
        }

        ret
        
    }
}

#[test]
fn test1(){
    let v = vec![-1,0,1,2,-1,-4];
    assert_eq!(Solution::three_sum(v), vec![vec![-1, -1]])
}


fn main() {
    println!("Hello world!");
}
