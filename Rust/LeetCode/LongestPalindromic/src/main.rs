struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<_> = s.chars().collect();

        let mut longest = (0, 0);

        let mut first = 0;
        let mut last = 0;

        let mut toggle = true;
        while last < chars.len() {
            println!("{first} {last}");
            let (i, j) = Self::palindrome_from_center(&chars, first, last).unwrap_or(longest);

            if j - i > longest.1 - longest.0 {
                longest = (i, j);
            }
            if toggle {
                last += 1;
            } else {
                first += 1;
            }
            toggle = !toggle;

        }

        return s[(longest.0)..=(longest.1)].to_string();
    }

    fn palindrome_from_center(chars: &Vec<char>, mut i1: usize, mut i2:usize) -> Option<(usize, usize)>{
        if chars[i1] != chars[i2] {
            return None;
        }

        loop  {
            if i1 > 0 && i2 < chars.len() - 1 && chars[i1- 1] == chars[i2 + 1]{
                i1 -= 1;
                i2 += 1;
            } else {
                return Some((i1, i2));
            }
        }
    }
}
fn main() {
    let s = String::from("babad");
    println!("{}", Solution::longest_palindrome(s));
}
