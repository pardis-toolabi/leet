
fn main() {
    let root = [3,9,20,null,null,15,7].to_vec();
    
    println!("Hello, world!");
}

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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
use std::rc::Rc;
use std::cell::RefCell;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    1
        
}


// Input: root = [3,9,20,null,null,15,7]
// Output: 3


// RefCell { value: TreeNode { val: 3, left: Some(RefCell { value: TreeNode { val: 9, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 20, left: Some(RefCell { value: TreeNode { val: 15, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 7, left: None, right: None } }) } }) } }))

