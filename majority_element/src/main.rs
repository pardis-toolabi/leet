use std::collections::HashMap;

fn main() {

    let l = [2,2,1,1,1,2,2,1,1,1,1].to_vec();
    println!("Hello, world!{:?}", get_majority(l));
}

fn get_majority(nums: Vec<i32>) -> i32{

    if nums.len() == 0 {
        return 0;
    }

    if nums.len() == 1 {
        return nums[0];
    }

    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut final_count = 0;
    let mut num =0;


    for i in nums{
        map.entry(i).and_modify(|c| *c +=1).or_insert(1);
        let count = map.get(&i).unwrap();

        if *count > final_count {
            num = i;
            final_count = *count;
        }
        
    }

    num

}

// Runtime
// 4
// ms
// Beats
// 5.48%
// Analyze Complexity
// Memory
// 2.36
// MB
// Beats
// 91.12%

fn get_majority_faster(nums: Vec<i32>) -> i32{
let mut count = 0;
let mut candidate = 0;

for num in nums {
    if count == 0 {
        candidate = num;
    }
    if num == candidate {
        count += 1;
    } else {
        count -= 1;
    }
}

candidate
}

// 0
// ms
// Beats
// 100.00%
// Analyze Complexity
// Memory
// 2.46
// MB
// Beats
// 49.35%


