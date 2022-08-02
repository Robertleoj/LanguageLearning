struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = nums.iter().map(|x|{x.pow(2)}).collect::<Vec<i32>>();
        ret.sort();
        ret
    }

}

fn main() {
    let a = vec![-4,-1,0,3,10];
    println!("{:?}", Solution::sorted_squares(a));
}
