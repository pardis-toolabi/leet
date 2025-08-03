fn main() {
    println!("Hello, world!");
    let triangle = [[-1].to_vec(),[2,3].to_vec(),[1,-1,-3].to_vec()].to_vec();
    let t = [[2].to_vec(),[3,4].to_vec(),[6,5,7].to_vec(),[4,1,8,3].to_vec()].to_vec();
    println!("{:?}!", minimum_total(triangle));
}
//    [-1]
//   [2,3]
// [1,-1,-3]
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut sum = triangle[0][0];
    let mut index = 0;
    for i in 0 .. triangle.len() - 1 {
        // let mut mins = [].to_vec();
        for j in 0 .. triangle[i].len(){
            if (sum + triangle[i+1][j]) < sum {
                sum = triangle[i+1][j];
                index = j;
            } 
            if (sum + triangle[i+1][j+1]) < sum {
                sum = triangle[i+1][j+1];
                index = j;
            }
        }
        println!("next value{:?}", mins);
        sum = mins.iter().min().unwrap().clone();   
    }
    sum 
}
