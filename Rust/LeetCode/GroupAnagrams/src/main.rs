
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for s in strs.iter() {
            let mut chars: Vec<_> = s.clone().chars().collect();
            chars.sort();
            hm.entry(chars).and_modify(|x| {x.push(s.to_string())}).or_insert(vec![s.to_string()]);
        }

        hm.into_values().collect()
    }
}
fn main() {
    let v = vec!["eat","tea","tan","ate","nat","bat"].iter().map(|x| {x.to_string()}).collect();
    println!("{:?}", Solution::group_anagrams(v));
}
