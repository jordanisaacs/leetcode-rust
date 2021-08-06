// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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
struct Solution{}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut answer_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut answer_head;

        let mut overflow = false;
        loop {
            let l1_val = match l1 {
                None => { 0 },
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
            };
            let l2_val = match l2 {
                None => { 0 }
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
            };
            let sum = l1_val + l2_val + if overflow {1} else {0};
            let sum = match sum >= 10 {
                true => {
                    overflow = true;
                    sum - 10
                }
                false => {
                    overflow = false;
                    sum
                }
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;

            if l1.is_none() && l2.is_none() && !overflow {
                break answer_head.unwrap().next;
            }


        }
    }
}
