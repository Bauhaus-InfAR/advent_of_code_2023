use std::fs;
use std::collections::HashMap;
use regex::Regex;
use num::integer::lcm;

const FILE_PATH: &str = "./input/input.txt";

fn main() {
    let re = Regex::new(r"^(?<k>\w+)\s+=\s+\((?<l>\w+),\s+(?<r>\w+)\)\s*$").unwrap();
    let lines = fs::read_to_string(FILE_PATH)
        .expect("File should be able to be read.");
    let mut lines = lines.trim()
        .lines();

    let directions = lines.next().unwrap().chars().collect::<Vec<char>>();
    let dir_length = directions.len();

    let mut steps = HashMap::new();
    for l in lines.skip(1) {
        let caps = re.captures(l).unwrap();
        let key = caps.name("k").unwrap().as_str();
        let left = caps.name("l").unwrap().as_str();
        let right = caps.name("r").unwrap().as_str();
        steps.insert(key, HashMap::from([('L', left), ('R', right)]));
    }

    let mut end = false;
    let mut n_steps = 0;
    let mut key = "AAA";
    while !end {
        let direction = &directions[n_steps % dir_length];
        n_steps += 1;
        key = steps[key][&direction];
        if key == "ZZZ" { end = true }
    }
    let task1 = n_steps;

    let mut keys = steps.keys().filter(|k| &k[2..3] == "A").collect::<Vec<&&str>>();
    keys.sort();
    let mut periodicity: Vec<usize> = vec![];
    for k in keys {
        key = k;
        n_steps = 0;
        loop {
            let direction = directions[n_steps % dir_length];
            n_steps += 1;
            key = steps[key][&direction];
            if &key[2..3] == "Z" { 
                periodicity.push(n_steps);
                break;
            }
        }
    }
    let task2 = lcm_rec(&periodicity);
    println!("Task 1: {task1}");
    println!("Task 2: {task2}");
}

fn lcm_rec(x: &Vec<usize>) -> usize {
    if x.len() == 2 { return lcm(x[0], x[1]) }
    let y = &x[0];
    lcm(lcm_rec(&Vec::from(&x[1..])), *y)
}