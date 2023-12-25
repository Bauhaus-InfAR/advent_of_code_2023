use std::fs;
use itertools::Itertools;

const FILE_PATH: &str = "./input/input.txt";

// struct
fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let space: (f64, f64) = (2e14, 4e14);
    // let space = (7., 27.);
    let mut task1: usize = 0;
    let data = contents
    .lines()
    .map(|l| {
        let parsed_line = l
        .split(" @ ")
        .map(|x| x
            .split(", ")
            .map(|e| e.trim().parse::<i128>().unwrap())
            .collect::<Vec<_>>()
        ).collect::<Vec<Vec<_>>>();
        let p1: (i128, i128, i128) = parsed_line[0].clone().into_iter().collect_tuple().unwrap();
        let p2: (i128, i128, i128) = (0..3).map(|i| parsed_line[0][i] + parsed_line[1][i]).collect_tuple().unwrap();
        (p1, p2)
    });

    let n = data.clone().count();

    data.clone().enumerate()
    .for_each(|(i, (ap1, ap2))| {
        let a1 = (ap1.0, ap1.1);
        let a2 = (ap2.0, ap2.1);
        let da = (a1.0 - a2.0, a1.1 - a2.1);
        for j in i+1..n {
            let (bp1, bp2) = data.clone().nth(j).unwrap();
            let b1 = (bp1.0, bp1.1);
            let b2 = (bp2.0, bp2.1);
            let db: (i128, i128) = (b1.0 - b2.0, b1.1 - b2.1);
            let xdiff = (da.0, db.0);
            let ydiff = (da.1, db.1);
            let div = det(xdiff, ydiff);
            if div != 0 {
                let d = (det(a1, a2), det(b1, b2));
                let x = det(d, xdiff) as f64 / div as f64;
                let y = det(d, ydiff) as f64 / div as f64;
                let within_space = x >= space.0 && x <= space.1 && y >= space.0 && y <= space.1;
                let in_future_a = x * -da.0 as f64  > (a1.0 * -da.0) as f64 && y * -da.1 as f64 > (a1.1 * -da.1) as f64;
                let in_future_b = x * -db.0 as f64  > (b1.0 * -db.0) as f64 && y * -db.1 as f64 > (b1.1 * -db.1) as f64;
                if within_space && // within space
                    in_future_a && in_future_b { // in the future
                    task1 += 1;
                }

            }  
        }

    });
    println!("Task 1: {}", task1);

}

fn det(a: (i128, i128), b: (i128, i128)) -> i128 {
    a.0 * b.1 - a.1 * b.0
}