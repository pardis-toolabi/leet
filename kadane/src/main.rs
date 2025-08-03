fn main() {
    let a = [-2,1,-3,4,-1,2,1,-5,4].to_vec();

    println!("Hello, world!{:?}", max_sub_array(a));

}



pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() == 0{
        return 0;
    }
    if nums.len() == 1{
        return nums[0];
    }


    let mut last_sum = nums[0];
    let mut max = 0;
    
    for i in 1 .. nums.len() {

        if nums[i] > nums[i]+last_sum{
            last_sum = nums[i];
            
        }else {
            last_sum = nums[i]+ last_sum;
            
        }

        if last_sum > max{
            max = last_sum
        }

    }


    max

        
}
