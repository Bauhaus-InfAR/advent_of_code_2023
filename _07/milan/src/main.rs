use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::convert::From;
use std::cmp::Ordering;

const FILE_PATH: &str = "./input/input.txt";

#[derive(Debug)]
struct Hand {
    cards: HashMap<u8, Vec<u8>>,
    bid: u32,
    value: HashMap<u8, u8>,
}

impl From<&str> for Hand {
    fn from(item: &str) -> Self {
        let item = item.split_ascii_whitespace().collect::<Vec<&str>>();
        let bid = item[1].parse::<u32>().unwrap();
        let task1_cards = item[0].chars().map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            other => other.to_digit(10).unwrap()
        } as u8).collect::<Vec<u8>>();
        let mut unique_cards = task1_cards.clone();
        unique_cards.sort();
        unique_cards.dedup();
        let mut freqs: BTreeMap<u8, u8> = BTreeMap::new();

        for c in unique_cards.iter() {
            freqs.insert(*c, task1_cards
            .iter()
            .filter(|n| *n == c)
            .count() as u8);
        };
        let mut sorted_freqs = freqs.values().collect::<Vec<&u8>>();
        sorted_freqs.sort();
        sorted_freqs.dedup();
        let number_of_pairs = freqs.values().filter(|v| *v == &2).count();
        let task1_value = get_hand_value(&sorted_freqs.last().unwrap(), &number_of_pairs);

        // --- TASK 2 --- //

        let mut joker_map: Vec<u8> = vec![];
        
        while joker_map.is_empty() {
            let tmp_freq = match sorted_freqs.pop() {
                Some(f) => f,
                None => break
            };            
            for v in freqs.iter() {
                if *v.0 != 11 && v.1 == tmp_freq {
                    joker_map.push(*v.0)
                }
            }            
        }
        
        if joker_map.is_empty() {joker_map.push(1)};
        joker_map.reverse();

        let mut task2_cards = task1_cards
            .iter()
            .map(|c| if *c == 11 { joker_map[0] } else {*c})
            .collect::<Vec<u8>>();
        unique_cards = task2_cards.clone();
        unique_cards.sort();
        unique_cards.dedup();
        freqs = BTreeMap::new();

        for c in unique_cards.iter() {
            freqs.insert(*c, task2_cards
            .iter()
            .filter(|n| *n == c)
            .count() as u8);
        };
        let number_of_pairs = freqs.values().filter(|v| *v == &2).count();
        let task2_value = get_hand_value(&freqs.values().max().unwrap(), &number_of_pairs);
        
        task2_cards = task1_cards
            .iter()
            .map(|c| if *c == 11 { 1 } else {*c})
            .collect::<Vec<u8>>();
        
        Self { cards: HashMap::from([(1, task1_cards), (2, task2_cards)]),
            bid: bid, 
            value: HashMap::from([(1, task1_value), (2, task2_value)])
        }
    }
}

fn get_hand_value(x: &u8, n_pairs: &usize) -> u8 {
    match x {
        1 => 0,
        2 => {
            if *n_pairs == 1 {
                1
            } else {
                2
            }
        },
        3 => {
            if *n_pairs == 1 {
                4
            } else {
                3
            }
        },
        other => *other + 1
    }
}
fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should be able to be read");
    
    let mut hands = contents.lines().map(|l| Hand::from(l)).collect::<Vec<Hand>>();
    hands.sort_by(|a, b| compare_hands(a, b, 0, 1));
    let task1 = (0..hands.len()).map(|i| {
        hands[i].bid * (i as u32 + 1)
    }).sum::<u32>();

    // --- TASK 2 --- //

    hands.sort_by(|a, b| compare_hands(a, b, 0, 2));
    let task2 = (0..hands.len()).map(|i| {
        hands[i].bid * (i as u32 + 1)
    }).sum::<u32>();

    println!("Task 1: {task1}");
    println!("Task 2: {task2}");
}

fn compare_hands(a: &Hand, b: &Hand, i: usize, task: u8) -> Ordering {
    match a.value[&task].cmp(&b.value[&task]) {
        Ordering::Less => a.value[&task].cmp(&b.value[&task]),
        Ordering::Greater => a.value[&task].cmp(&b.value[&task]),
        Ordering::Equal => compare_cards(a, b, i, task)
    }
}

fn compare_cards(a: &Hand, b: &Hand, i: usize, task: u8) -> Ordering {
    match a.cards[&task][i].cmp(&b.cards[&task][i]) {
        Ordering::Less => a.cards[&task][i].cmp(&b.cards[&task][i]),
        Ordering::Greater => a.cards[&task][i].cmp(&b.cards[&task][i]),
        Ordering::Equal => if i==4 { Ordering::Equal } else { compare_cards(a, b, i+1, task)}
    }
}