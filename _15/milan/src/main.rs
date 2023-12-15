use std::fs;
use std::collections::HashMap;

#[cfg(windows)]
const LINE_ENDING : &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING : &'static str = "\n";

const FILE_PATH: &str = "./input/input.txt";

fn main() {
    let mut boxes: HashMap<usize, HashMap<&str, (usize, usize)>> = HashMap::new();
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let text = contents.replace(LINE_ENDING, "");
    let task1 = text.split(",").fold(0,|acc: usize, x| acc + hash(x));
    println!("Task 1: {}", task1);

    // --- TASK 2 --- //
    text.split(",").for_each(|x| {
        let rm = x.contains("-");
        let separator = match rm {
            true => "-",
            false => "="            
        };
        let lens = x.split(separator).collect::<Vec<_>>();
        let lens_label = lens[0];
        let box_number = hash(lens_label) + 1;
        match rm {
            true => {
                boxes.entry(box_number).and_modify(|e| {e.remove(lens_label);});
            },
            false => {
                boxes.entry(box_number)
                .and_modify(|e| {
                    let order = match e.get(lens_label) {
                        Some(x) => x.1,
                        None => match e.iter().map(|a| a.1.1).max() {
                                Some(i) => i + 1,
                                None => 0
                            }
                    };
                    e.insert(lens_label, (lens[1].parse().unwrap(), order));
                })
                .or_insert(HashMap::from([(lens_label, (lens[1].parse().unwrap(), 0))]));
            }
        };
    });
    let mut focusing_power: Vec<usize> = vec![];
    for i in 1..=256 {
        let this_box = match boxes.get(&i) {
            Some(x) => x,
            None => continue
        };
        let mut lenses = this_box.iter().map(|l| *l.1).collect::<Vec<_>>();
        lenses.sort_by(|a, b| a.1.cmp(&b.1));
        lenses.iter().enumerate().for_each(|(j, l)| focusing_power.push(i * (j + 1) * l.0))
    }
    let task2 = focusing_power.iter().sum::<usize>();
    println!("Task 2: {}", task2);

}

fn hash(a: &str) -> usize {
    a.bytes().fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
}