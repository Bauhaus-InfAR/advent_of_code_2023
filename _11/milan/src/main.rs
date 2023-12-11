use std::fs;

const FILE_PATH: &str = "./input/input.txt";
// set correct factor for task
// task 1
// const EXPANSION_FACTOR: usize = 2;

// task 2
const EXPANSION_FACTOR: usize = 10_usize.pow(6);

fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let mut grid: Vec<Vec<char>> = vec![];
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];
    for (i, l) in contents.lines().enumerate() {
        grid.push(l.chars().collect());
        if l.chars().all(|c| c == '.') {
            empty_rows.push(i)
        }
    }
    
    for i in 0..grid[0].len() {
        let mut column = grid.iter().map(|r| r[i]);
        if column.all(|e| e == '.') {
            empty_cols.push(i);
        }
    }
    let mut galaxies: Vec<(usize, usize)> = vec![];

    for (i, l) in grid.iter().enumerate() {
        let row_expand = empty_rows.iter().filter(|c| *c < &i).count();
        for (j, c) in l.iter().enumerate() {  
            let col_expand = empty_cols.iter().filter(|r| *r < &j).count();
            if *c == '#' {
                galaxies.push((i + row_expand * (EXPANSION_FACTOR - 1), j + col_expand * (EXPANSION_FACTOR - 1)));
            }
        }
    }
    let mut distances: usize = 0;
    for i in 0..galaxies.len()-1 {
        for j in i+1..galaxies.len() {
            distances += dist(&galaxies[i], &galaxies[j]);
        }
    }
    println!("{:#?}", distances);
}

fn dist(x: &(usize, usize), y: &(usize, usize)) -> usize {
    x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
}
