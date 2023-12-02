use std::fs;
use std::collections::HashMap;
use regex::Regex;

const FILE_PATH: &str = "./input/input.txt";

fn main() {
    let max_draws: HashMap<&str, u16> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let game_regex: Regex = Regex::new(r"^\s*Game\s+(?<game>\d+)\s*").unwrap();
    let draw_regex: Regex = Regex::new(r"^\s*(?<n>\d+)\s+(?<colour>\w+)\s*").unwrap();
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    let mut all_game_numbers: Vec<u16> = vec![];
    let mut possible_games: Vec<u16> = vec![];
    let mut game_powers: Vec<u16> = vec![];
    for line in contents.lines() {
        let mut segments = line.split(&[':', ';']);
        let game_string = segments.next().unwrap();
        let caps = game_regex.captures(game_string).unwrap();
        let game_number = caps["game"].parse::<u16>().unwrap();
        all_game_numbers.push(game_number);
        let mut game: Vec<HashMap<&str, u16>> = vec![];
        let mut is_game_possible = true;
        let mut min_balls_possible: HashMap<&str, u16> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for round in segments {
            let mut draw: HashMap<&str, u16> = HashMap::new();
            ["red", "green", "blue"].map(|x| draw.insert(x, 0));
            let draws = round.split(", ");
            for d in draws {
                for caps in draw_regex.captures_iter(d) {                        
                    let colour = caps.name("colour").unwrap().as_str();
                    let n = caps.name("n").unwrap().as_str().parse::<u16>().unwrap();
                    draw.insert(&colour, n);
                    if n > *max_draws.get(&colour).unwrap() {
                        is_game_possible = false;
                    }
                };
            }
            for colour in ["red", "green", "blue"] {
                let current = draw.get(colour).unwrap();
                if current > min_balls_possible.get(colour).unwrap() {
                    min_balls_possible.insert(colour, *current);
                }
            };
            game.push(draw);
        }
        if is_game_possible {
            possible_games.push(game_number);
        }
        game_powers.push(
            min_balls_possible
                .values()
                .cloned()
                .collect::<Vec<_>>()
                .iter()
                .copied()
                .reduce(|a,b| a * b)
                .unwrap()
        )
    }
    let task1: u16 = possible_games.iter().sum();
    let task2: u16 = game_powers.iter().sum();
    println!("Task 1 result: {}", task1);
    println!("Task 2 result: {}", task2);
}
