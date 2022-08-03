struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {

        let chars: Vec<_> = haystack.chars().collect();
        let needle_chars: Vec<_> = needle.chars().collect();

        for i in 0..chars.len() {
            let mut j = i;
            loop {
                if j - i + 1 > needle_chars.len() {
                    return i as i32;
                }

                if j >= chars.len() {
                    return -1;
                }

                if chars[j] == needle_chars[j - i] {
                    j += 1;
                } else {
                    break;
                }
            }
        }

        -1
    }
}

fn main() {
    println!("{}", Solution::str_str("hello".to_string(), "ll".to_string()));
    println!("{}", Solution::str_str("hello".to_string(), "".to_string()));
    println!("{}", Solution::str_str("aaa".to_string(), "aaaa".to_string()));
}
