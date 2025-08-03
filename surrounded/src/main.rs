fn main() {
    let mut i = [['X','X','X','X'].to_vec(),['X','O','O','X'].to_vec(),['X','X','O','X'].to_vec(),['X','O','X','X'].to_vec()].to_vec();
    // println!("{:?}", i);
    let mut j = [['X','O','X'].to_vec(),['X','O','X'].to_vec(),['X','O','X'].to_vec()].to_vec();
    solve(&mut i);
    println!("i: {:?}", i);
    solve(&mut j);
    println!("j: {:?}", j);
}


pub fn solve(board: &mut Vec<Vec<char>>) {

    if board.len() == 0 || board.len() == 1 {
        return; 
    }

    for i in 0 .. board.len() {
        if board[i][0] == 'O'{
            flag(i, 0, board, 's');

        }
        if board[i][board[0].len()- 1] == 'O' {
            flag(i, board[0].len()- 1, board, 's');

        }

    }

    for j in 0 .. board[0].len() {
        if board[0][j] == 'O'{
            flag(0, j, board, 's');

        }
        if board[board.len() - 1 ][j] == 'O' {
            flag(board.len() - 1 , j, board, 's');

        }

    }

    for i in 0 .. board.len(){
        for j in 0 .. board[1].len() {
            if board[i][j] == 'O' {
                board[i][j] = 'X';
            }
            if board[i][j] == 's' {
                board[i][j] = 'O';
            }
        }
    }
    
}

fn flag(i: usize, j:usize,  board: &mut Vec<Vec<char>>, flag_char: char){

    if board[i][j] == flag_char {
        return;
    }
    board[i][j] = flag_char;

    if i > 0 {
        if board[i-1][j] == 'O'{
            flag(i-1, j, board, flag_char);
        }
    }
    if i <  board.len() - 1{
        if board[i+1][j] == 'O'{
            flag(i+1, j, board, flag_char);
        }
    }
    if j > 0 {
        if board[i][j-1] == 'O'{
            flag(i, j-1, board, flag_char);
        }
    }
    if j < board[0].len() - 1 {
        if board[i][j+1] == 'O'{
            flag(i, j+1, board, flag_char);
        }
    }

    
}

