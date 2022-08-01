
struct Solution{
    first_bad: i32
}

impl Solution {

    pub fn first_bad_version(&self, n: i32) -> i32 {
		// Do binary search on badVersion
        
        let mut high: i64 = n as i64;
        let mut low: i64 = 1;

        if n == 1 {
            return 1;
        }

        if self.isBadVersion(1) {
            return 1;
        }
        
        loop {
            if high - 1 == low {
                return high as i32;
            }

            let mid = (high + low) / 2;
            // println!("{low} {mid} {high}");

            match self.isBadVersion(mid as i32){
                true => {
                    high = mid;
                },
                false => {
                    low = mid;
                }
            }

        }
    }

    fn isBadVersion(&self, k: i32)-> bool{
        return k >= self.first_bad;
    }

    fn new(k: i32) -> Self{
        return Solution{
            first_bad: k
        }
    }

}

fn main() {
    let s = Solution::new(1702766719);

    let n = s.first_bad_version(2126753390);

    println!("{}", n);

}

