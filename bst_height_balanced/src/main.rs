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
use std::collections::VecDeque;
pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{


    let mut cur = head.as_ref();
    let mut nums = Vec::new();

    if let Some(node) = cur {
        nums.push(node.val);
        cur = node.next.as_ref()
    }
    println!("Hello, world!{:?}", nums);
    nums.sort();

    let mut cur = head.as_mut();


    let new = ListNode::new(0);

    let mut list = nums.into_iter();
    while let (Some(m), Some(n)) = (cur,list.next() ){
        m.val = n;
        cur = m.next.as_mut()

    }
    head
}


fn main() {
    let a = 11 / 2;
    let h =Some(Box::new(ListNode::new(0)));
    sort_list(h);
    // println!("Hello, world!{:?}", a);
}


