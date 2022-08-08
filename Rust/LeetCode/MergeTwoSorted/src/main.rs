struct Solution;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

macro_rules! str {
    ($($x:expr), *) => ($($x.to_string()), *);
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //pseudonode
        let mut head =  Box::<ListNode>::new(ListNode{val:0,next:None});

        let mut tail =  &mut head;

        let mut l1 = &list1;
        let mut l2 = &list2;

        loop {
            match (l1, l2) {
                (Some(a), None) | (None, Some(a)) => {
                    tail.next = Some(a.to_owned());
                    return head.next;
                },
                (None, None) => {return head.next;},
                (Some(a), Some(b)) => {

                    if a.val <= b.val {
                        tail.next = Some(a.to_owned());
                        tail = tail.next.as_mut().unwrap();
                        l1 = &l1.as_ref().unwrap().next;
                    } 

                    if b.val <= a.val {
                        tail.next = Some(b.to_owned());
                        tail = tail.next.as_mut().unwrap();
                        l2 = &l2.as_ref().unwrap().next;
                    }
                }
            }
            
        }
        
    }
}


fn main() {
    println!("Hello world!");
}
