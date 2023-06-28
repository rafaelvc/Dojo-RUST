//https://leetcode.com/problems/add-two-numbers/
//https://leetcode.com/problems/add-two-numbers/solutions/1433177/rust-0ms/?q=rust&orderBy=most_relevant
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

//1
//1 6
//1 5
//---
// 3 1
pub mod add_two_numbers{
    use crate::add_two_numbers::ListNode;

    pub fn sum(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if ! l2.is_some() { return l1 }
        if ! l1.is_some() { return l2 }
        //let mut nl1 = &l1;
        //let mut nl2 = &l2;
        let mut nl1 = l1.as_ref(); // as_ref because it is not mutable, it is read only access
        let mut nl2 = l2.as_ref();



        // Essse result é um node extra pois se fosse fazer
        // result = result.next  (o compilador entenderia como uma cópia
        // então ele a pede implementação de um Copy trait para o <Option<Box<ListNode>>>)
        // Dessa forma evita o Copy trait e devolve o próximo result.next no retorno que
        // é o real inicio da Lista
        let mut result = Some(Box::new(ListNode::new(0)));

        let mut prev = result.as_mut(); // is a mutable reference, result will be changed (.next)
        let mut dez = 0;
        while nl1.is_some() && nl2.is_some() {
//            let sum = nl1.as_ref().unwrap().val + nl2.as_ref().unwrap().val + dez;
            let sum = nl1.unwrap().val + nl2.unwrap().val + dez;
            dez = sum / 10;
            prev.as_deref_mut().unwrap().next = Some( Box::new(ListNode::new(sum % 10)) );
            prev = prev.unwrap().next.as_mut();
            nl1 = nl1.unwrap().next.as_ref();
            nl2 = nl2.unwrap().next.as_ref();
//            nl1 = &(nl1.as_ref().unwrap().next);
//            nl2 = &(nl2.as_ref().unwrap().next);
        }
        if nl2.is_some() {nl1 = nl2}
        while nl1.is_some() {
            //let sum = nl1.as_ref().unwrap().val + dez;
            let sum = nl1.unwrap().val + dez;
            dez = sum / 10;
            prev.as_deref_mut().unwrap().next = Some( Box::new(ListNode::new(sum % 10)) );
            prev = prev.unwrap().next.as_mut();
            nl1 = nl1.unwrap().next.as_ref();
            //nl1 = &(nl1.as_ref().unwrap().next);
        }
        if dez > 0 {
            prev.as_deref_mut().unwrap().next = Some( Box::new(ListNode::new( dez )) );
        }
        result.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::add_two_numbers::sum;
    use crate::add_two_numbers::ListNode;
    #[test]
    fn test_sum() {

        let mut l1 = Some(Box::new(ListNode::new(2)));
        l1.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        l1.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        let mut l2 = Some(Box::new(ListNode::new(5)));
        l2.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
        l2.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

        let mut r = Some(Box::new(ListNode::new(7)));
        r.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
        r.as_deref_mut().unwrap().next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(8)));

        assert_eq!(r, sum (l1, l2) );

        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(0)));
        let r = Some(Box::new(ListNode::new(0)));

        assert_eq!(r, sum (l1, l2) );

        //[9,9,9,9,9,9,9]
        let mut l1 = Some(Box::new(ListNode::new(9)));
        l1.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        let mut n = l1.as_deref_mut().unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));

        // [9,9,9,9]
        let mut l2 = Some(Box::new(ListNode::new(9)));
        l2.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        n = l2.as_deref_mut().unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));

        //[8,9,9,9,0,0,0,1]
        let mut r = Some(Box::new(ListNode::new(8)));
        r.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        n = r.as_deref_mut().unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
        n = n.unwrap().next.as_mut();
        n.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(1)));

        assert_eq!(r, sum (l1, l2) );

    }
}
