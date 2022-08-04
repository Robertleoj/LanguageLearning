use std::collections::{BTreeMap, btree_map::Iter};

struct Solution;

impl Solution {
    fn subsets_rec(bm: &BTreeMap<i32, i32>, mut it: Iter<i32, i32>) -> Option<Vec<Vec<i32>>> {

        let (k, repeats) = it.next()?;
        match Self::subsets_rec(bm, it) {
            Some(v) => {
                Some(
                    (0..=*repeats).map(|count| {
                        v.clone().into_iter().map(|x| {[vec![*k;count as usize], x].concat()}).collect()
                    }).collect::<Vec<Vec<Vec<i32>>>>().concat()
                )
            },
            None => {
                Some((0..=*repeats).map(|count| {vec![*k;count as usize]}).collect())
            }
        }
        
    }

    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut bm = BTreeMap::new();

        for n in nums {
            bm.entry(n)
                .and_modify(|e| {*e += 1})
                .or_insert(1);
        }

        let mut i = bm.iter();

        Self::subsets_rec(&bm, i).unwrap_or(vec![])
    }
}

fn main() {
    let v = vec![1, 2, 2];
    println!("{:?}", Solution::subsets_with_dup(v));
}
