use std::fs;

const NUMBERS: &'static [&str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
fn main() {
    let file_path = "./input/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut values = vec![];
    for line in contents.lines() {
        let mut digits: Vec<String> = vec![];
        for start in 0..line.len() {
            let this_char = line.chars().nth(start).unwrap_or('x');
            if this_char.is_digit(10) {
                digits.push(this_char.to_string());
                continue;
            }
            for end in start+1..=line.len() {
                let my_slice = &line[start..end].to_lowercase();
                if !could_be_number(my_slice) {
                    break;
                }
                let digit = str_to_number(my_slice);
                if digit != "NaN" {
                    digits.push(digit);
                    break;
                }
            }
        }
        let vec_length = digits.len();
        if vec_length > 0 {
            let first = &digits[0];
            let last = &digits[vec_length - 1];
            let val = format!("{}{}", first, last);
            let val = val.parse().expect("parse panic");
            values.push(val);
        } else {
            values.push(0);
        };
    }
    let res: u32 = values.iter().sum();
    println!("{res}");
}

fn could_be_number(x: &str) -> bool {
    if x.len() == 0 {
        return false;
    }
    NUMBERS.iter().any(|n| n.starts_with(x))
}

fn str_to_number(x: &str) -> String {
    match  NUMBERS.iter().position(|&n| n == x) {
        None => String::from("NaN"),
        Some(n) => {
           n.to_string()
        }
    }
}