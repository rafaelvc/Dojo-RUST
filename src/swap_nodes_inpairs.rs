//https://leetcode.com/problems/swap-nodes-in-pairs/
#![allow(dead_code)]

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

pub mod swap_nodes_inpairs{ 
    use crate::swap_nodes_inpairs::ListNode;
    


    pub fn swap(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        // let mut x = head;
        // let mut y = x.as_mut().unwrap().next.take(); // as_mut otherwise compiler consider we are moving x

        // //let new_head = y;
        // let mut p = Some(Box::new(ListNode::new(0)));
        
        // x.as_mut().unwrap().next = y.as_mut().unwrap().next.take();
        // y.as_mut().unwrap().next = x;
        // //if p.as_ref().unwrap().val != 0 {
        // //  p.as_mut().unwrap().next = y;
        // //}
        // p = x;


        //result
        //Some(Box::new(ListNode::new(0)))

        // https://leetcode.com/problems/swap-nodes-in-pairs/solutions/502696/rust-0ms-2-1mb-without-recursion/

        let mut root = Box::new(ListNode {val:0,next:head});
        let mut tail = &mut root;
        loop {
            let mut front = if tail.next.is_none() {
                break;
            } else {
                tail.next.take().unwrap()
            };
            let mut back = if front.next.is_none() {
                tail.next = Some(front);
                break;
            } else {
                front.next.unwrap()
            };
            front.next = back.next.take();
            back.next = Some(front);
            tail.next = Some(back);
            tail = tail.next.as_mut().unwrap();
            tail = tail.next.as_mut().unwrap();
        }
        root.next

    }
}

#[cfg(test)]
mod tests {
    use crate::swap_nodes_inpairs::ListNode;
    use super::swap_nodes_inpairs::swap;
    #[test]
    fn test_palindrome() {
        let r = Some(Box::new(ListNode::new(0)));
        assert_eq!(r, swap(Some(Box::new(ListNode::new(0)))));
    }
}