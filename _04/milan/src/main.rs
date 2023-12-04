use std::fs;
use regex::{Regex, Captures};

const FILE_PATH: &str = "./input/input.txt";

fn main() {
    let re = Regex::new(r"^Card\s+(?<card>\d+):\s(?<winning>[\d ]+) \|\s+(?<numbers>[\d ]+)").unwrap();
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should be possible to read");
    let mut points: Vec<u16> = vec![];
    let mut n_winning: Vec<usize> = vec![];
    let lines = contents.lines();
    let mut n_cards = vec![1; lines.clone().count()];
    for (i, l) in lines.enumerate() {
        let caps = re.captures(l).unwrap();
        let winning = parse_card_numbers(&caps, "winning");
        let actual = parse_card_numbers(&caps, "numbers");
        let mut hits: Vec<&u8> = vec![]; 
        for i in actual.iter() {
            if winning.contains(i) {
                hits.push(i)
            }
        };
        let n_hits = hits.len();
        n_winning.push(n_hits);
        if n_hits > 0 {
            points.push(2_u16.pow((n_hits - 1) as u32));
            for j in 1..=n_hits {
                n_cards[i+j] += n_cards[i];
            };
        }
    }
    let task1 = points.iter().sum::<u16>();
    let task2 = n_cards.iter().sum::<u32>();
    println!("{}", task1);
    println!("{}", task2);
}

fn parse_card_numbers(captures: &Captures<'_>, name: &str) -> Vec<u8> {
    captures
        .name(name)
        .unwrap()
        .as_str()
        .split_ascii_whitespace()
        .map(|x| x
            .parse::<u8>()
            .unwrap()
        )
        .collect::<Vec<u8>>()
}