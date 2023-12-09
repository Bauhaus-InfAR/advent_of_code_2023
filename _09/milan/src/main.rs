use std::fs;

const FILE_PATH: &str = "./input/input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let task = contents
        .lines()
        .map(|l| {
            let line = l
            .split_ascii_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
            find_next(&line)
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();
    println!("Task 1: {}", task.1);
    println!("Task 2: {}", task.0);
}

fn find_next(x: &Vec<i32>) -> (i32, i32) {
    if x.iter().all(|e| *e == 0) { return (0, 0) };
    let diffs = x.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    let nxt = find_next(&diffs);
    (x.first().unwrap() - nxt.0, x.last().unwrap() + nxt.1)
}