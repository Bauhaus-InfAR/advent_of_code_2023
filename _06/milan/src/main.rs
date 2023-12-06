use std::fs;

const FILE_PATH: &str = "./input/input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should be able to be read");
    
    // --- TASK 1 --- //

    let parsed = contents
        .trim()
        .split('\n')
        .map(|l| l
            .split_ascii_whitespace()
            .skip(1)
            .map(|n| n.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
        )
        .flatten()
        .collect::<Vec<u16>>();   

    let n_races = parsed.len()/2;
    let mut races = vec![];
    for i in 0..n_races {
        let race = (parsed[i], parsed[i+n_races]);
        races.push((1..race.0)
            .map(|f| {
                let dist = (race.0 - f) * f;
                dist
            })
            .filter(|r| r > &race.1)
            .count()
        );
    }
    let task1 = races.into_iter().reduce(|a, b| a * b).unwrap();

    println!("Task 1: {task1}");

    // --- TASK 2 --- //

    let parsed = contents
        .trim()
        .split('\n')
        .map(|l| l
            .split_ascii_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .unwrap()
        )
        .collect::<Vec<u64>>();
    
    let mut break_loop = false;
    let mut task2 = 0;
    for i in parsed[1]/parsed[0]..parsed[0] {
        let res = i * (parsed[0] - i);
        if res > parsed[1] {
            task2 += 1;
            break_loop = true;
        } else if break_loop {
            break;
        }
    }
    
    println!("Task 2: {task2}");

    // smart solution //
    
    /* just a quadratic equation

    time * speed = dist
    (total_time - push_time) * push_time - dist = 0
    tt*t0 - t0^2 - d = 0
    -t0^2 + tt*t0 - d = 0
    ax^2 + bx + c = 0
    a = -1
    x = t0
    b = tt
    c = -d

    t0 = (-b +/- sqrt(b^2 - 4ac)) / 2a
    t0 = (-tt + sqrt(tt^2 - 4 * -1 * -d)) / (2 * -1)
    t0 = -(tt - sqrt(tt^2 - 4*d)) / -2
    t0 = (tt - sqrt(tt^2 - 4*d)) / 2

    ans = time - (2 * t0 + 1)

    */ 

    let tt = parsed[0];
    let d = parsed[1];

    let t0 = (tt - ((tt * tt - 4 * d) as f64).sqrt() as u64) / 2;
    let task2 = tt - (2 * t0 - 1); // -1 due to some fuckery with integer arithmetic
    println!("Task 2 (smart): {task2}");
}
