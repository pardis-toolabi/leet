fn main() {
    let input = [
        ['1', '1', '1', '1', '0'].to_vec(),
        ['1', '1', '0', '1', '0'].to_vec(),
        ['1', '1', '0', '0', '0'].to_vec(),
        ['0', '0', '0', '0', '0'].to_vec(),
    ];
    println!("{:?}", num_islands(input.to_vec()));
}


use std::collections::VecDeque;
impl Solution {


pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.len() == 0 {
        return 0;
    }

    let g: Vec<Vec<char>> = grid.clone();
    let mut visited: Vec<Vec<i32>> = Vec::new();
    let mut counter = 0;

    for i in 0..g.clone().len() {
        for j in 0..g[0].clone().len() {
            println!("{:?}{:?}", i, j);
            if g[i][j] == '1' && !visited.contains(&[i as i32, j as i32].to_vec()) {
                Self::BFS(
                    i as i32,
                    j as i32,
                    &mut visited,
                    g.len() as i32,
                    g[0].len() as i32,
                    g.clone(),
                );
                // println!("i, j): {:?} {:?}", i, j);
                counter += 1;
            }
        }
    }

    counter
}

fn BFS(i: i32, j: i32, v: &mut Vec<Vec<i32>>, m: i32, n: i32, g: Vec<Vec<char>>) {

    let mut q = VecDeque::new();
    q.push_back([i, j].to_vec());
    v.push([i, j].to_vec());


    while !q.is_empty() {
        let c: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

        if let Some(index) = q.pop_front() {

            for k in c {
                let new_i = index[0] + k[0];
                let new_j = index[1] + k[1];

                let range_ckeck = (new_i < m && new_i >= 0) && (new_j < n && new_j >= 0);
                let visited_check = !v.contains(&[new_i, new_j].to_vec());
                if range_ckeck && g[(new_i) as usize][(new_j) as usize] == '1' && visited_check {
                    q.push_back([new_i, new_j].to_vec());
                    v.push([new_i, new_j].to_vec());
                }
            }

        }

    }
}

}