use std::collections::{linked_list, LinkedList};

fn main() {
// create a linked list
#[derive(PartialEq)]
struct ListNode {
    pub value : Option<i32>,
    pub next: Option<Box<ListNode>>
}

impl ListNode {

    fn new(value:Option<i32>)-> Self {
        ListNode{
            value,
            next : None
        }
    }
    
}
#[derive(PartialEq)]
struct LinkedList {
    head : Option<ListNode>
}

impl LinkedList {

    fn new(values: Vec<i32>) -> Self {
        let mut cur = ListNode::new(None);
        for i in 0 .. values.len(){
            if cur.next == None {
                cur.next = Some(Box::new(ListNode{value : Option<values[i]>, next: None}));
            }else {
                node[i].next =  cur.next;
                cur.next = nodei
            }
        }
        LinkedList{ 
            head:None
        }

    }
    
}
}


// impl Solution {
//     pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {   
//         println!("list1{:?}", list1);
//         println!("list2{:?}", list2);
//         match (list1, list2){
//         match (l1, l2) {
//             (None, Nonce) => None,
//             (None, None) => None,
//             (Some(n), None) | (None, Some(n)) => Some(n),
//             (Some(n), None) | (None, Some(n)) => Some(n),
//             (Some(list1), Some(list2)) => {
//                 if list1.val >= list2.val {
//                     Some(Box::new(ListNode{
//                         val: list2.val,
//                         next: Solution::merge_two_lists(Some(list1), list2.next)
//                     }))
//                 }else{
//                     Some(Box::new(ListNode{
//                     val: list1.val,
//                     next: Solution::merge_two_lists(list1.next, Some(list2))
//                     }))   
//                 }
//             }
//         }
        
//     }
// }


// impl Solution {
//     pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         match (l1, l2) {
//             (None, None) => None,
//             (Some(n), None) | (None, Some(n)) => Some(n),
//             (Some(l1), Some(l2)) => {
//                 if l1.val >= l2.val {
//                     Some(Box::new(ListNode {
//                         val: l2.val,
//                         next: Solution::merge_two_lists(Some(l1), l2.next)
//                     }))
//                 } else {
//                     Some(Box::new(ListNode {
//                         val: l1.val,
//                         next: Solution::merge_two_lists(l1.next, Some(l2))
//                     }))
//                 }
//             }
//         }
//     }
// }