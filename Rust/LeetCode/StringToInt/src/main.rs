struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars: Vec<_> = s.trim().chars().collect();

        let mut digits = Vec::<i32>::new();

        let mut i = 0;

        let sign = if chars.len() > 0 {
            match chars[0] {
                '-' => {i += 1; true},
                '+' => {i += 1; false},
                _ => false
            }
        } else {
            false
        };

        let mut len = 0;
        while i < chars.len() {
            let n = match chars[i] {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => {break;}
            };

            digits.push(n);
            i += 1;
            len += 1;
        }

        if digits.len() == 0 {
            return 0;
        }

        let mut out: i32 = 0;

        for (idx, n) in digits.iter().enumerate() {

            let num = n.saturating_mul(
                    (10 as i32).saturating_pow(len - (idx as u32) - 1)
                ).saturating_mul(if sign {-1} else {1});

            out = out.saturating_add(num);
        }

        out
        

    }
}


fn main() {
    println!("{}", Solution::my_atoi("42".to_string()));
    println!("{}", Solution::my_atoi("+42".to_string()));
    println!("{}", Solution::my_atoi("-42".to_string()));
    println!("{}", Solution::my_atoi("-49090002abdre".to_string()));
    println!("{}", Solution::my_atoi("".to_string()));
}
