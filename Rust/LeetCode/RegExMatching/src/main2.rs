struct Solution;

use std::cmp::{max, min};

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

macro_rules! str {
    ($($x:expr), *) => ($($x.to_string()), *);
}

impl Solution {
    fn is_match_rec(s: &Vec<char>, p: &Vec<char>, sidx: usize, mut pidx: usize) -> bool {
        if sidx == s.len() && pidx == p.len() {
            return true;
        }

        if pidx == p.len() {
            return false;
        }

        println!("At {sidx} {pidx}\n");

        let mut maxln = 0;
        let mut minln = 0;


        let mut chars = Vec::new();
        let mut at_chars = false;
        let mut n_symbols = 0;

        while pidx < p.len() {
            match &p[pidx] {
                '*' if !at_chars => {n_symbols += 1;maxln = p.len()},
                '.' if !at_chars => {n_symbols += 1;minln += 1;maxln+=1;},
                '*' | '.' if at_chars => break,
                c => {at_chars = true; chars.push(c)},
            }
            pidx += 1;
        }

        println!("After while loop\n{sidx} {pidx}\n");
        println!("minln: {minln} maxln: {maxln}");

        if chars.len() == 0 && n_symbols > 0{
            let nleft = s.len() - sidx;
            return nleft <= maxln && nleft >= minln;
        }

        println!{"chars: {:?}", chars};

        // here we know there are some characters we need to match, 


        let s_char_maxidx = min(sidx + maxln, s.len() - 1);
        let s_char_minidx = min(sidx + minln, s.len() - 1);

        println!("minidx: {s_char_minidx} maxidx: {s_char_maxidx}");

        'mloop: for i in s_char_minidx..=s_char_maxidx {
            let mut found = true;

            for idx in i..(i + chars.len()) {
                if idx == s.len() {
                    break 'mloop;
                }

                if chars[idx - i] != &s[idx] {
                    found = false;
                    break;
                } 
            }

            if found && Self::is_match_rec(s, p, i + chars.len(), pidx) {
                return true;
            }
        }

        println!("\n\n");
        false

    }

    pub fn is_match(s: String, p: String) -> bool {

        let siter: Vec<char> = s.chars().collect();
        let piter: Vec<char> = p.chars().collect();
        Self::is_match_rec(&siter, &piter, 0, 0)
    }
}


fn main() {
    println!("----------------------------------");
    let s = "abc";
    let p = "a";
    assert!(!Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");

    let s = "aa";
    let p = "a*";
    assert!(Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");

    let s = "hallo eg er ekki heima";
    let p = "h*er.e*k. he*m*";
    assert!(Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");


    let s = "hallo eg er ekki heima";
    let p = "*er.e*k. he*m*a.";
    assert!(!Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");

    let s = "hallo eg er ekki heima";
    let p = "*er.e*k. he*m*a.";
    assert!(!Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");


    // "aab"
    // "c*a*b"
}
