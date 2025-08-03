use std::cmp::{max, min};

fn main() {
    let input  = [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]];
    let input = input.iter().map(|&x| x.to_vec()).collect();
    // let a = [[4,3].to_vec(),[1,2].to_vec()].to_vec();
    println!("{:?}", snakes_and_ladders(input));
}

pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let mut counter = 0;

    // start from the bottom left
    // choose next in range [curr + 1, min(curr + 6, n2)]
    // if next != -1 its snake or latter so follow
    // the number of the steps matter
    // return the least number of dice rolls
    // for evry choice we check the max 
    // if its not a snake we go through

    let mut flag = false;
    let mut flat = Vec::new();
    for i in (0.. board[0].len() ).rev(){
        if flag{
            for j in (0 .. board[0].len()).rev(){
                flat.push(board[i][j]);
    
            }
        }else {
            for j in 0 .. board[0].len(){
                flat.push(board[i][j]);
    
            }
        }
        flag = !flag;
    }
    
    choose_next(flat, &mut counter);

    counter
        
}


fn choose_next(board: Vec<i32>, counter: &mut i32) {
    // choose the best next step
    // choose max in [curr + 1, min(curr + 6, n2)]
    // if it has a snake move one behind until it doesnt
    let mut cur = 0;

    println!("board.clone().len()-1  {:?}", board.clone().len()-1 );
    println!("board {:?}", board);

    while cur < board.clone().len()-1 {

        // let next_step_max = max(cur + 1, min(cur + 6, board.clone().len()));
        println!("111111111111111111111");
        println!("cur {:?}", cur);
        // println!("diff {:?}", (min(cur + 6, board.clone().len())) - (cur +1));
        
        // let mut n_t = Vec::new();
        let mut biggest = cur;
        let mut last_i = 0;
        let mut smallest = cur;
        for i in cur .. (min(cur + 6, board.clone().len())) {
            if board[i] > biggest as i32{
                biggest = board[i] as usize;
            }
            if board[i] == -1 {
                last_i = i;
            }
            if board[i] != -1 && board[i] < smallest as i32{
                smallest = board[i] as usize;
            }

            // println!("i{:?}", i);
            // n_t.push(board[i]);
        }
            println!("cur + 1: {:?}", cur );
            println!("til: {:?}", min(cur + 6, board.clone().len()));
            println!("biggest: {:?}", biggest);
            println!("last_i: {:?}", last_i);
            println!("smallest: {:?}", smallest);

            println!("is bigger : {:?}", biggest > cur );
            println!("is sameee: {:?}", last_i != 0 );



        if biggest > cur {
            cur = biggest as usize;
        }else if last_i != 0 {
            cur = last_i;
        }else {
            cur = smallest;
        }
        last_i = 0;
        // n_t.sort();
        // let max = n_t[n_t.len()-1];
        // if max != -1 && max > cur as i32 {
        //     cur = max as usize;

        // }
        // if max == -1 {
        //     cur = cur + 6;
        // }
        // if max < cur as i32{
        //     cur = cur +5;
        // }
        // n_t = [].to_vec();

        *counter +=1;
        
    }


}

fn get_non_snake( cur: usize, board: &Vec<i32>)-> usize{

    if cur>0 && cur < board.len(){
        if board[cur] != -1 && board[cur] > cur as i32{
            let new_index = board[cur] as usize;
            get_non_snake(new_index, &board);
        }
        if board[cur] != -1 && board[cur] < cur as i32{
            let new_index = cur -1 ;
            get_non_snake(new_index, &board);
        }
    }

    cur
}

// board [-1, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 35, -1, 
// -1, 13, -1, -1, -1, -1,
//  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]cur/ 15
// cur + 1: 16
// til: 21
// biggest: 15
// last_i: 20
// smallest: 18446744073709551615

// // board [-1, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 35, -1,
//  -1, 13, -1, -1, -1, -1, 
-1, -1, -1, -1, -1, -1,
 -1, -1, -1, -1, -1, -1,
  -1, -1, -1]