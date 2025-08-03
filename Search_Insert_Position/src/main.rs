fn main() {
    println!("Hello, world!");

}




pub fn search_insert(mut nums: Vec<i32>, target: i32) -> i32 {

    if !nums.contains(&target) {
        nums.push(target);
    }

    nums.sort();

    let mut pos = 0;

    for (i, item) in nums.iter().enumerate() {

        if *item == target {
            pos = i;
        }
    }

    pos as i32


}

