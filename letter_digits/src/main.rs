use std::collections::HashMap;

fn main() {
    // println!("{:?}", letter_combinations("23".to_string()));
    // letter_combinations("23".to_string());
    println!("{:?}", combine(4,2));
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    let letters = vec![
        [].to_vec(),
        [].to_vec(),
        ['a','b','c'].to_vec(),
        ['d','e','f'].to_vec(),
        ['g','h','i'].to_vec(),
        ['j','k','l'].to_vec(),
        ['m','n','o'].to_vec(),
        ['p','q','r','s'].to_vec(),
        ['t','u','v'].to_vec(),
        ['w','x','y','z'].to_vec()
    ];

    if digits.len() == 0 {
        return vec![];
    };

    let mut comb: Vec<String> = vec!["".to_string()];
    for char in digits.chars() {
        let mut curr_comb: Vec<String>  = Vec::new();
        if let Some(digit) = char.to_digit(10){
            let char_letters = &letters[digit as usize];
            // println!("{:?}", char_letters);
            for j in comb {
                for i in char_letters{
                    curr_comb.push(j.clone() + &i.to_string() );
                }

            }
        }
        println!("{:?}", curr_comb);
        comb = curr_comb;
    }
    // println!("{:?}", comb);
    comb
}


pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    // k is the len of the inside vec
    //  1 .. n
    
    let mut out = Vec::new();
    for i in 1 .. n+1 {
        println!("{:?}", i);

        // let mut cur = Vec::new();
        for j in i + 1 .. n - k + 1{
            let mut mid_cur = Vec::new();
            mid_cur.push(i);
            for k in 1 .. k {
                mid_cur.push(j+k);
            }
            println!("{:?}", mid_cur.clone());

            out.push(mid_cur.clone())
        }
        // out.push(cur);

    }

    out
    
}