use std::{fs, isize, vec};

const FILE_PATH: &str = "./input/input.txt";

fn main() { 
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let grid = contents.lines().map(|l| l
        .chars().collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    let mut all_n_energised: Vec<usize> = vec![];

    let nrows = grid.len() - 1;
    let ncols = grid[0].len() - 1;
    for i in 0..nrows {
        let row = i as isize;
        let mut n_energised: usize = 0;
        let mut visited: Vec<[isize; 2]> = vec![];
        let mut paths: Vec<([isize; 2], [isize; 2])> = vec![];
        trace(&[row, 0], &grid, &[0, 1], &mut paths, &mut visited, &mut n_energised);
        all_n_energised.push(n_energised);
        n_energised = 0;
        paths = vec![];
        visited = vec![];
        trace(&[row, ncols as isize], &grid, &[0, -1], &mut paths, &mut visited, &mut n_energised);
        all_n_energised.push(n_energised);
    }

    for i in 0..ncols {
        let col = i as isize;
        let mut n_energised: usize = 0;
        let mut paths: Vec<([isize; 2], [isize; 2])> = vec![];
        let mut visited: Vec<[isize; 2]> = vec![];
        trace(&[0, col], &grid, &[1, 0], &mut paths, &mut visited, &mut n_energised);
        all_n_energised.push(n_energised);
        n_energised = 0;
        paths = vec![];
        visited = vec![];
        trace(&[nrows as isize, col], &grid, &[-1, 0], &mut paths, &mut visited, &mut n_energised);
        all_n_energised.push(n_energised);
    }
    
    let task1 = all_n_energised[0];
    println!("Task 1: {}", task1);
    let task2 = all_n_energised.iter().max().unwrap();
    println!("Task 2: {}", task2);

}

fn trace(coord: &[isize; 2], data: &Vec<Vec<char>>, direction: &[isize; 2], path: &mut Vec<([isize; 2], [isize; 2])>, visited: &mut Vec<[isize; 2]>, n_energised: &mut usize) -> () {
    let [mut r, mut c] = coord;
    let [mut dr, mut dc] = direction;
    
    let height = data.len() as isize;
    let width = data[0].len() as isize;
    
    loop {
        if !(r >= 0 && r < width && c >= 0 && c < height) {
            break;}
        let this_step = ([r, c], [dr, dc]);
        if path.contains(&this_step) {break}
        if !visited.contains(&this_step.0) {
            *n_energised += 1;
            visited.push(this_step.0.clone());
        };
        path.push(this_step);

        let current_char = data[r as usize][c as usize];

        if (current_char == '-' && dr == 0) || (current_char == '|' && dc == 0) || current_char == '.' {
        } else if current_char == '/' {
            (dr, dc) = (-dc, -dr);
        } else if current_char == '\\' {
            (dr, dc) = (dc, dr);
        } else {
            trace(&[r.clone() + dc.clone(), c.clone() + dr.clone()], data, &[dc.clone(), dr.clone()], path, visited, n_energised);
            trace(&[r.clone() - dc.clone(), c.clone() - dr.clone()], data, &[-dc.clone(), -dr.clone()], path, visited, n_energised);
            break;
        }
        r += dr;
        c += dc;
    }
}
