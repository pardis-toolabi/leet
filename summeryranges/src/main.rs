use std::collections::{hash_map, HashMap, HashSet};

fn main() {
    let nums = [0,1,2].to_vec();
    println!("{:?}", summary_ranges(nums));
}
// [0,1,2,4,5,7]
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 || nums.len()==1 {
        "".to_string();
    }
    // check to see if next number is +1 
    let mut out :Vec<String> = vec![];
    let mut starts :Vec<i32> = vec![];
    let mut ends :Vec<i32> = vec![];
    let mut h:HashMap<i32, i32>= HashMap::new();


    for (i, item) in nums.iter().enumerate() {
        let mut num1 = item.clone();
        let is = h.get(item);
        let is = h.insert(num1, 0);
        println!("is: {:?}", is);
        let mut c = 0; 
        println!("num1: {:?}", nums[i+c]);
        while i+c+1 < nums.len() &&  nums[i+c] + 1 == nums[i+c+1]  && is == None{
            println!("while num1{:?}", num1);
            num1 = nums[i+c+1] ;
            h.insert(nums[i+c+1], 0);
            c += 1;
        }
        if 
        starts.push(item.clone());
        ends.push(num1);
        println!("after while: {:?}", num1.clone());

    }


    let mut result : Vec<String> =vec![]; 
    println!("sta{:?}", starts);
    println!("end{:?}", ends);


    for i in 0 .. starts.len(){
        if starts[i] == ends[i] {
            let f = format!("{}",starts[i]);
            result.push(f);
        }else{
            let f = format!("{}->{}", starts[i], ends[i]);
            result.push(f);
        }
    }

    result
    
}
