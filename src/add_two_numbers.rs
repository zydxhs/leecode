#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() || l2.is_none() {
            return None;
        }

        let (mut l1, mut l2, mut header) = (l1, l2, Some(Box::new(ListNode::new(0))));
        let (mut flag, mut node) = (0, &mut header);

        while l1.is_some() || l2.is_some() || flag != 0 {
            let e1 = if let Some(e) = l1 {
                l1 = e.next;
                e.val
            } else {
                0
            };
            let e2 = if let Some(e) = l2 {
                l2 = e.next;
                e.val
            } else {
                0
            };

            let val = e1 + e2 + flag;
            let val = if val > 9 {
                flag = 1;
                val - 10
            } else {
                flag = 0;
                val
            };

            let now = Some(Box::new(ListNode::new(val)));
            if let Some(e) = node {
                e.next = now;
                node = &mut e.next;
            }
        }

        header.unwrap().next
    }

    pub fn add_two_numbers_2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() || l2.is_none() {
            return None;
        }

        let (mut l1, mut l2, mut header) = (l1, l2, Some(Box::new(ListNode::new(0))));
        let (mut flag, mut node) = (0, &mut header);

        while l1.is_some() || l2.is_some() || flag != 0 {
            match (l1, l2) {
                (Some(e1), Some(e2)) => {
                    let mut e = e1.val + e2.val + flag;
                    if e > 9 {
                        flag = 1;
                        e -= 10;
                    } else {
                        flag = 0;
                    }
                    let cur = Box::new(ListNode::new(e));
                    if let Some(e) = node {
                        e.next = Some(cur);
                    }
                    node = &mut node.as_mut().unwrap().next;
                    l1 = e1.next;
                    l2 = e2.next;
                }
                (Some(e1), None) => {
                    let mut e = e1.val + flag;
                    if e > 9 {
                        flag = 1;
                        e -= 10;
                    } else {
                        flag = 0;
                    }
                    let cur = Box::new(ListNode::new(e));
                    if let Some(e) = node {
                        e.next = Some(cur);
                    }
                    node = &mut node.as_mut().unwrap().next;
                    l1 = e1.next;
                    l2 = None;
                }
                (None, Some(e2)) => {
                    let mut e = e2.val + flag;
                    if e > 9 {
                        flag = 1;
                        e -= 10;
                    } else {
                        flag = 0;
                    }
                    let cur = Box::new(ListNode::new(e));
                    if let Some(e) = node {
                        e.next = Some(cur);
                    }
                    node = &mut node.as_mut().unwrap().next;
                    l1 = None;
                    l2 = e2.next;
                }
                (None, None) => {
                    if flag > 0 {
                        let cur = Box::new(ListNode::new(flag));
                        flag = 0;
                        if let Some(e) = node {
                            e.next = Some(cur);
                        }
                        node = &mut node.as_mut().unwrap().next;
                    }
                    l1 = None;
                    l2 = None;
                }
            };
        }

        header.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn1() {
        let l1 = Some(Box::new(ListNode { val: 2, next: None }));
        let l1 = Some(Box::new(ListNode { val: 4, next: l1 }));
        let l1 = Some(Box::new(ListNode { val: 3, next: l1 }));

        let l2 = Some(Box::new(ListNode { val: 5, next: None }));
        let l2 = Some(Box::new(ListNode { val: 6, next: l2 }));
        let l2 = Some(Box::new(ListNode { val: 4, next: l2 }));

        let ret = Some(Box::new(ListNode { val: 8, next: None }));
        let ret = Some(Box::new(ListNode { val: 0, next: ret }));
        let ret = Some(Box::new(ListNode { val: 7, next: ret }));
        assert_eq!(ret, Solution::add_two_numbers(l1, l2));

        let l1 = Some(Box::new(ListNode { val: 6, next: None }));
        let l1 = Some(Box::new(ListNode { val: 4, next: l1 }));
        let l1 = Some(Box::new(ListNode { val: 8, next: l1 }));

        let l2 = Some(Box::new(ListNode { val: 5, next: None }));
        let l2 = Some(Box::new(ListNode { val: 6, next: l2 }));
        let l2 = Some(Box::new(ListNode { val: 7, next: l2 }));

        let ret = Some(Box::new(ListNode { val: 1, next: None }));
        let ret = Some(Box::new(ListNode { val: 2, next: ret }));
        let ret = Some(Box::new(ListNode { val: 1, next: ret }));
        let ret = Some(Box::new(ListNode { val: 5, next: ret }));
        assert_eq!(ret, Solution::add_two_numbers(l1, l2));
    }

    #[test]
    fn test_fn2() {
        let l1 = Some(Box::new(ListNode { val: 2, next: None }));
        let l1 = Some(Box::new(ListNode { val: 4, next: l1 }));
        let l1 = Some(Box::new(ListNode { val: 3, next: l1 }));

        let l2 = Some(Box::new(ListNode { val: 5, next: None }));
        let l2 = Some(Box::new(ListNode { val: 6, next: l2 }));
        let l2 = Some(Box::new(ListNode { val: 4, next: l2 }));

        let ret = Some(Box::new(ListNode { val: 8, next: None }));
        let ret = Some(Box::new(ListNode { val: 0, next: ret }));
        let ret = Some(Box::new(ListNode { val: 7, next: ret }));
        assert_eq!(ret, Solution::add_two_numbers_2(l1, l2));

        let l1 = Some(Box::new(ListNode { val: 6, next: None }));
        let l1 = Some(Box::new(ListNode { val: 4, next: l1 }));
        let l1 = Some(Box::new(ListNode { val: 8, next: l1 }));

        let l2 = Some(Box::new(ListNode { val: 5, next: None }));
        let l2 = Some(Box::new(ListNode { val: 6, next: l2 }));
        let l2 = Some(Box::new(ListNode { val: 7, next: l2 }));

        let ret = Some(Box::new(ListNode { val: 1, next: None }));
        let ret = Some(Box::new(ListNode { val: 2, next: ret }));
        let ret = Some(Box::new(ListNode { val: 1, next: ret }));
        let ret = Some(Box::new(ListNode { val: 5, next: ret }));
        assert_eq!(ret, Solution::add_two_numbers_2(l1, l2));
    }
}
