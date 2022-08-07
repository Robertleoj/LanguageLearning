struct Solution;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

macro_rules! str {
    ($($x:expr), *) => ($($x.to_string()), *);
}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ret = String::new();
        let strs : Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();

        let mut i = 0;
        loop {
            let mut currchar = 'c';
            let mut initialized = false;
            for s in &strs {
                if s.len() <= i {return ret;}

                if !initialized {
                    currchar = s[i]; 
                    initialized = true;
                } else if s[i] != currchar {
                    return ret;
                }
            }
            ret.push(currchar);
            i += 1;
        }
    }
}

#[test]
fn test1(){
    let v = strvec!["flower","flow","flight"];
    assert_eq!(Solution::longest_common_prefix(v), "fl");
}


fn main() {
}
