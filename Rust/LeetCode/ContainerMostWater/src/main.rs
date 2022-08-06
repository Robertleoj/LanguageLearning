struct Solution;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

macro_rules! str {
    ($($x:expr), *) => ($($x.to_string()), *);
}

use std::cmp::{min, max, Ordering};

impl Solution {

    
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut res = 0;
        while left < right {
            let (hleft, hright) = (height[left], height[right]);
            res = max(res, (right - left) as i32 * min(hleft, hright));
            match hleft.cmp(&hright) {
                Ordering::Greater => right -= 1,
                Ordering::Less => left += 1,
                Ordering::Equal => {right -= 1; left += 1}
            }
        }

        res

    }
}

fn main() {
    println!("Hello world!");
}


#[cfg(test)]
mod test {

    use super::*;

    use std::fs;

    fn perform_test(v: Vec<i32>, solution: i32){
        if v.len() < 30 {
            println!("{:?}", v);
        }
        assert_eq!(Solution::max_area(v), solution);
    }

    #[test]
    fn test1(){
        perform_test(vec![1, 1], 1);
    }

    #[test]
    fn test2(){
        perform_test(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49);
    }

    #[test]
    fn test3() {
        let v: Vec<i32> = fs::read_to_string("test.txt").unwrap()
            .split(",").map(|x| x.to_string())
            .map(|x|x.parse::<i32>().expect("error parsing"))
            .collect();
        perform_test(v, 500);

    }
}