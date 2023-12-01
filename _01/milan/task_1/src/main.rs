use std::fs;

fn main() {
    let file_path = "./input/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut values = vec![];
    for line in contents.lines() {
        let digits: Vec<char> = line
            .chars()
            .filter(|s| s.is_digit(10))
            .collect();
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
