use std::{ fs, cmp };
use regex::Regex;

const FILE_PATH: &str = "./input/input.txt";

#[derive(Debug)]
struct Gear {
    position: (usize, usize),
    factors: Vec<u32>,
}

impl Gear {
    fn add_factor(&mut self, x: u32) {
        self.factors.push(x);
    }
}

struct Digit {
    value: char,
    gear_position: Vec<(usize, usize)>
}

impl Digit {
    fn add_gear_position(&mut self, x: usize, y: usize) {
        self.gear_position.push((x, y))
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    factor_of: Vec<(usize, usize)>
}
fn main() {
    let dot_regex = Regex::new(r"\.").unwrap();
    // let symbol_re = Regex::new(r"[^\d]").unwrap();
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    let text_lines: Vec<&str> = contents.lines().collect();
    let n_lines = text_lines.len();
    let line_length = text_lines[0].len();
    let mut gears: Vec<Gear> = vec![];
    let mut all_numbers: Vec<Number> = vec![];
    for (i, l) in text_lines.iter().enumerate() {
        let mut this_line = l.to_string();
        this_line.push('.');
        let mut this_number: Vec<Digit> = vec![];
        let mut number_counts = false;
        for (j, c) in this_line.chars().enumerate() {
            if c == '*' {
                gears.push(Gear {position: (i, j), factors: vec![]});
            }
            if !c.is_numeric() {
                if number_counts {
                    let mut this_number_value = String::new();
                    let mut this_number_gears: Vec<(usize, usize)> = vec![];
                    for d in this_number.iter() {
                        this_number_value.push(d.value);
                        let mut gear_positions = d.gear_position.clone();
                        this_number_gears.append(&mut gear_positions);
                    }
                    this_number_gears.dedup();
                    all_numbers.push(Number { value: this_number_value.parse().unwrap(), factor_of: this_number_gears});
                    number_counts = false;
                }
                this_number = vec![];
                continue;
            }
            let mut this_digit = Digit {value: c, gear_position: vec![]};
            let kernel = get_kernel(text_lines.clone(), i, j, n_lines, line_length);
            for row in 0..3 {
                let kernel_row = &kernel[row];
                for (col, ch) in kernel_row.chars().enumerate() {
                    if ch == '*' {
                        this_digit.add_gear_position(i + row - 1, j + col - 1);
                    }
                }
            }
            this_number.push(this_digit);
            let mut str_from_kernel = kernel.join("");
            str_from_kernel = dot_regex.replace_all(&str_from_kernel, "0").to_string();
            if !str_from_kernel.chars().all(char::is_numeric) {
                number_counts = true;
            };
        };
    };
    let task1 = all_numbers.iter().map(|n| n.value).collect::<Vec<u32>>().iter().sum::<u32>();
    for g in gears.iter_mut() {
        for n in all_numbers.iter() {
            if n.factor_of.contains(&g.position) {
                g.add_factor(n.value);
            };
        }
    }
    // let iter = gears.iter().filter(|g| g.factors.len() == 2);
    let mut gear_ratios: Vec<u64> = vec![];
    for g in gears {
        if g.factors.len() != 2 {continue;}
        gear_ratios.push(u64::from(g.factors[0]) * u64::from(g.factors[1]));
    }
    let task2 = gear_ratios.iter().copied().reduce(|a, b| a + b).unwrap();
    println!("Task 1: {task1}");
    println!("Task 2: {task2}");

}

fn get_kernel(text_vec: Vec<&str>, line: usize, pos: usize, max_l: usize, max_p: usize) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    let x_start = if pos == 0 {0} else {pos-1};
    let x_end = cmp::min(pos+2, max_p);
    let y_start = if line == 0 {0} else {line-1};
    let y_end = cmp::min(line+2, max_l);
    if line == 0 {out.push(String::from("..."))};
    for i in y_start..y_end {
        let mut this_line = String::from(&text_vec[i][x_start..x_end]);
        if pos == 0 {
            this_line = pad(this_line, true);
        }
        if pos == max_p {
            this_line = pad(this_line, false);
        }
        out.push(this_line);
    }
    if line + 1 == max_l {out.push(String::from("..."))};
    out
}

fn pad(x: String, front: bool) -> String {
    let mut out = String::new();
    if front {
        out.push('.');
    }
    for c in x.chars() {out.push(c)}
    if !front {
        out.push('.');
    };
    out
}