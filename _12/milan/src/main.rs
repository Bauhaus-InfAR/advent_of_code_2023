use std::fs;
use itertools::Itertools;

const FILE_PATH: &str = "./input/input.txt";
const FACTOR: usize = 1; // task 2 Factor = 5
struct Row {
    chars: Vec<char>,
    nums: Vec<u8>
}

fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let mut all_combinations = vec![];
    contents.lines().for_each(|l| {
        print!("\rProcessing line: {l}");
        let mut row_combinations = 0;
        let row = l.split_ascii_whitespace().collect::<Vec<_>>();
        
        let chars = row[0].chars();
        let mut row = Row {
            chars: chars.clone().collect(),
            nums: row[1].split(",").map(|x| x.parse::<u8>().unwrap()).collect()
        };
        if FACTOR > 1 {
            row.chars.push('?');
            row.nums = row.nums.iter()
            .cycle()
            .take(row.nums.len() * FACTOR)
            .map(|x| *x)
            .collect::<Vec<_>>();
        };

        let expanded_row = row.chars
            .iter()
            .cycle()
            .take(row.chars.len() * FACTOR)
            .collect::<Vec<_>>();        
        let n_unknown = expanded_row.iter().filter(|c| ***c == '?').count();
        let combs = get_combinations(n_unknown);
        for comb in combs {            
            let mut replaced_row: Vec<char> = vec![];
            expanded_row.iter().for_each(|x| replaced_row.push(**x));
            
            let mut j = 0;
            for (i, c) in expanded_row.iter().enumerate() {
                if **c == '?' {
                    replaced_row[i] = comb[j];
                    j += 1;
                }
            }
            let replaced_row = replaced_row.iter().collect::<String>();
            let groups = replaced_row.split('.').filter(|g| g.len() > 0).map(|g| g.len());
            
            if groups.clone().count() != row.nums.len() {continue; }
            let is_match = groups.enumerate().all(|(i, g)| g == row.nums[i % row.nums.len()] as usize); 
            if !is_match { continue; }
            row_combinations += 1;
        }
        all_combinations.push(row_combinations);
        println!(": {row_combinations}");
    }
    );
    let task = all_combinations.iter().sum::<i32>();
    println!("\nTask: {}", task);
}

fn get_combinations(n: usize) -> Vec<Vec<char>> {
    (1..=n).map(|_| ".#".chars()).multi_cartesian_product().unique().collect::<Vec<_>>()
}