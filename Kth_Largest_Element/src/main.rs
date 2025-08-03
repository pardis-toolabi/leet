use core::num;
use std::i32;

fn main() {

    let a =[-1,2,0];

    println!("Hello, world!{:?}", find_kth_largest(a.to_vec(), 3));
}



pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut v = i32::MAX;

    for _ in 0 .. k as usize {
        let mut max  =i32::MIN;
        let mut max_index = 0;
        for (i, item) in nums.iter().enumerate() {
            if *item > max {
                max = *item;
                max_index = i;
            }

        }
        
        if max < v {
            v= max;
        } 
        nums[max_index] = i32::MIN;
    }
    v
}