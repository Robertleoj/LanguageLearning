// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {

    fn rec(vec: &Vec<i32>, s: usize, e:usize) -> Option<Rc<RefCell<TreeNode>>> {
        println!("{s} {e}");

        let mid = (e + s) / 2;

        let left = if mid > s {
            Self::rec(vec, s, mid - 1)
        } else {
            None
        };

        let right = if mid < e {
            Self::rec(vec, mid + 1, e)
        } else {
            None
        };

        Some(Rc::new(RefCell::new(TreeNode {
            val: vec[mid],
            left,
            right
        })))
        
    }
    
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::rec(&nums, 0, nums.len() - 1)
    }
}

fn main() {
    let v = vec![-10,-3,0,5,9];

    println!("{:#?}", Solution::sorted_array_to_bst(v));



}
