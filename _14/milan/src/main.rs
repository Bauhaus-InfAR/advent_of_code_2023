use std::fs;
use std::collections::HashMap;


const FILE_PATH: &str = "./input/input.txt";
const N_CYCLES: i32 = 1000_000_000;
fn main() {
    let mut period = 0;
    let mut rot_cache: HashMap<String, (i32, Vec<String>)> = HashMap::new();
    let mut col_cache: HashMap<String, String> = HashMap::new();
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let mut grid = contents.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let nrows = grid.len();
    
    let mut period_after = 0;
    let mut task1 = 0;
    for c in 0..N_CYCLES {
        let key = grid.join("\n");
        if rot_cache.contains_key(&key) {
            grid = rot_cache.get(&key).unwrap().1.clone();
            period = c;
            period_after = rot_cache.get(&key).unwrap().0;
            break;
        }
        for r in 0..4 {
            let ncols = grid[0].len();
            grid = (0..ncols).map(|i| {
                let column = grid.iter().map(|r| &r[i..i+1]).collect::<String>();
                match col_cache.get(&column) {
                    Some(x) => return x.to_string(),
                    None => {
                    let tilted = tilt(&column, false);
                    col_cache.insert(column, tilted.clone());
                    return tilted;
                    }
                };
            }).collect::<Vec<_>>();
            if c == 0 && r == 0 {
                task1 = grid.iter()
                    .map(| l| l
                        .chars()
                        .enumerate()
                        .map(|(i, c)| if c == 'O' {i+1} else {0})
                        .sum::<usize>())
                    .sum::<usize>();
            }          
        }  
        rot_cache.insert(key, (c + 1, grid.clone())); 
    }
    let cycles_to_do = (N_CYCLES - period_after) % (period - period_after + 1);
    for _c in 0..cycles_to_do {
        for _ in 0..4 {
            let ncols = grid[0].len();
            grid = (0..ncols).map(|i| {
                let column = grid.iter().map(|r| &r[i..i+1]).collect::<String>();
                
                    let tilted = tilt(&column, false);
                    col_cache.insert(column, tilted.clone());
                    return tilted;
                    
            }).collect::<Vec<_>>();        
        } 
    }
    let task2 = grid.iter()
        .enumerate()
        .map(|(i, l)| l
            .chars().filter(|c| *c == 'O').count() * (nrows - i))
        .sum::<usize>();
    println!("Task 1: {:#?}", task1);
    println!("Task 2: {:#?}", task2);
}

fn tilt(column: &str, inverse: bool) -> String  {
    let mut pattern = String::from(".O");
    if inverse {pattern = pattern.chars().rev().collect::<String>()};
    let repl = pattern.chars().rev().collect::<String>();
    let mut x = column.clone().to_string();
    while x.contains(&pattern) {
       x = x.replace(&pattern, &repl);
    }
    x.chars().rev().collect()
}