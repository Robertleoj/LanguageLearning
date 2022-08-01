use std::cmp::Ordering;

struct Solution;


impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        
        let mut low:i32 = 0;
        let mut high: i32 = (nums.len() as i32) - 1;

        loop {

            let mid = (high + low) / 2;

            if mid == high {
                if nums[mid as usize] >= target {
                    return mid;
                } else {
                    return mid + 1;
                }
            }

            if high == low {
                return low;
            }

            match target.cmp(&nums[mid as usize]){
                Ordering::Less => {
                    high = mid;
                },
                Ordering::Greater => {
                    low = mid + 1;
                },
                Ordering::Equal => {
                    return mid;
                }

            }

        }
    }
}

fn main() {

    let nums = [1];


    println!("{}",Solution::search_insert(nums.to_vec(), 1));
}
