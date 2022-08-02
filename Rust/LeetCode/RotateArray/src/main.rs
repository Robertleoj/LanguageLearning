struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k % nums.len() as i32;


        let rightpart = &nums[..(nums.len() - k as usize)];

        let leftpart = &nums[(nums.len() - k as usize)..];


        *nums = [leftpart, rightpart].concat().to_vec();

    }
}

fn main() {
    let mut arr = vec![1,2,3,4,5,6,7];
    let k = 3;
    Solution::rotate(&mut arr, k);

    println!("{:?}", arr);
}
