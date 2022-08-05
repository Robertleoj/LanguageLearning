struct Solution;

macro_rules! str {
    ($($x:expr), *) => ($($x.to_string()), *);
}


use std::collections::HashMap;

impl Token {
    fn from_string(s: String) -> Vec<Token> {
        let mut vec = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;

        while i < s.len() {
            vec.push(
                if i < s.len() - 1 && chars[i + 1] == '*' {
                    i += 1;
                    match chars[i - 1] {
                        '.' => Token::AnyMatch,
                        a => Token::Match(a),
                    }
                } else {
                    match chars[i] {
                        '.' => Token::Any,
                        a => Token::Char(a),
                    }
                }
            );
            i += 1;
        }
        vec
    }
}


#[derive(Debug)]
enum Token {
    Any,
    AnyMatch,
    Match(char),
    Char(char),
}

impl Solution {
    fn is_match_rec(s: &Vec<char>, p: &Vec<Token>, sidx: usize, pidx: usize, cache: &mut HashMap<(usize, usize), bool>) -> bool {

        match cache.get(&(sidx, pidx)).map(|entry| entry.clone()) {
            Some(res) => res,
            None => {
                if sidx == s.len() && pidx == p.len() {
                    return true;
                }

                if pidx == p.len() {
                    return false;
                }

                if sidx == s.len() {
                    return match p[pidx] {
                        Token::Match(_) | Token::AnyMatch => Self::is_match_rec(s, p, sidx, pidx + 1, cache),
                        _ => false
                    }
                }

                let res = match p[pidx] {
                    Token::Any => Self::is_match_rec(s, p, sidx + 1,pidx + 1, cache),
                    Token::Char(c) => s[sidx] == c && Self::is_match_rec(s, p, sidx + 1,pidx + 1, cache),
                    Token::Match(c) => {
                        (c == s[sidx] && Self::is_match_rec(s, p, sidx + 1, pidx, cache))
                        || Self::is_match_rec(s, p, sidx, pidx + 1, cache)
                    },
                    Token::AnyMatch => {
                        Self::is_match_rec(s, p, sidx + 1, pidx, cache)
                        || Self::is_match_rec(s, p, sidx, pidx + 1, cache)
                    }
                };

                cache.insert((sidx, pidx), res.clone());

                res
            }
        }



    }

    pub fn is_match(s: String, p: String) -> bool {

        let schars = s.chars().collect();
        let tokenseq = Token::from_string(p);

        println!("{:?}", schars);
        println!("{:?}", tokenseq);

        let mut hm = HashMap::new();

        Self::is_match_rec(&schars, &tokenseq, 0, 0, &mut hm)
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

    let s = "hhhhhe";
    let p = "h*..e";
    assert!(Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");

    let s = "aab";
    let p = "c*a*b";
    assert!(Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");

    let s = "ab";
    let p = ".*";
    assert!(Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");

    let s = "a";
    let p = "ab*";
    assert!(Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");

    let s = "mississippi";
    let p = "mis*is*p*.";
    assert!(!Solution::is_match(str!(s), str!(p)));
    println!("Correct ({s}) ({p})");
    println!("----------------------------------");


    // 
    // 
}
