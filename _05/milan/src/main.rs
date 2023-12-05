use std::collections::HashMap;
use std::fs;
use regex::Regex;

const FILE_PATH: &str = "./input/input.txt";


fn main() {
    let re = Regex::new(r"(?s)map:\s+([\d+\s+]+)").unwrap();
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should be able to be read");
    let mut mapping: Vec<Vec<Vec<i64>>> = vec![vec![]; 7];
    let mut lines = contents.lines();
    let seeds = lines.next().unwrap().split(" ").skip(1).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    

    for (i, (_, [x])) in re.captures_iter(&contents).map(|c| c.extract()).enumerate() {
        for l in x.lines() {
            let line_vec = l
                .split_ascii_whitespace()
                .map(|x| x
                    .parse::<i64>()
                    .unwrap())
                .collect::<Vec<i64>>();
            if line_vec.len() == 0 { continue; }
            mapping[i].push(line_vec);
        }
    
    }
    let task1 = seeds
        .iter()
        .map(|s| translate(*s, &mapping))
        .min()
        .unwrap();

    let mut task2_seeds = (0..seeds.len()).step_by(2).map(|s| {
        HashMap::from([("start", seeds[s]), ("end", seeds[s] + seeds[s+1])])
    }).collect::<Vec<HashMap<&str, i64>>>();
    task2_seeds.sort_by(|a, b| a["start"].cmp(&b["start"]));
 
    let max_n: i64 = 999999999;
    
    let mut location: i64 = 0;
    'outer: for l in 0..=max_n {
        if l % 10000000 == 0 {
            println!("location: {l}");
        }
        location = l;
        let target = back_translate(&l, &mapping);
        
        for s in &task2_seeds {
            if target >= s["start"] && target <= s["end"] {
                break 'outer;
            }
        }
    }
    
    let task2 = location;

    println!("Task 1: {task1}");
    println!("Task 2: {task2}");
}

fn translate_step(mut x: i64, dict: &Vec<Vec<i64>>) -> i64 {

    for d in dict.iter() {
        if x >= d[1] && x < d[1] + d[2] {
            let dist = d[0] - d[1];
            x += dist;
            break;
        }
    }
    x
}

fn translate(mut x: i64, dict_vec: &Vec<Vec<Vec<i64>>>) -> i64 {
    for i in 0..dict_vec.len() {
        x = translate_step(x, &dict_vec[i]);
    }
    x
}

fn back_translate(x: &i64, dict_vec: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut out = *x;
    for i in (0..dict_vec.len()).rev() {
        for d in dict_vec[i].iter() {
            if out >= d[0] && out < d[0] + d[2] {
                let dist = d[1] - d[0];
                out += dist;
                break;
            }
        }
    }
    out
}
