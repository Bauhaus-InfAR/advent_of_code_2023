use std::{fs, iter::{zip, Zip}};
use std::collections::VecDeque;

const FILE_PATH: &str = "../input/input.txt";

fn main() {
    let contents: String = fs::read_to_string(FILE_PATH)
    .expect("File should load");
    let lines: Vec<(char, f64, &str)> = contents.lines().map(|l| {
        let mut split = l.split_ascii_whitespace();
        let dir = split.next().unwrap().chars().nth(0).unwrap();
        let length = split.next().unwrap().parse::<f64>().unwrap();
        let color = split.next().unwrap();
        (dir, length, color)
    }
    ).collect::<Vec<_>>();
    let mut path = VecDeque::from(vec![(0.5, 0.5, 0., 0.)]);
    lines.iter().enumerate().for_each(|(i, (_dir, _len, hex))| {
        let mut hex = hex.replace("(#", "");
        hex = hex.replace(")", "");
        let dir = hex.chars().last().unwrap();
        let hex_len = hex.chars().take(5).collect::<String>();
        let len = i32::from_str_radix(&hex_len, 16).unwrap() as f64;
        // println!("{}", len);
        let (dr, dc) = match dir {
            '1' => {
                println!("{:?}", ('D', len));
                (len, 0.)
            },
            '3' => {
                println!("{:?}", ('U', len));
                (-len, 0.)
            },
            '0' => {
                println!("{:?}", ('R', len));
                (0., len)
            },
            '2' => {
                println!("{:?}", ('L', len));
                (0., -len)
            },
            _ => (0., 0.)
        };
        let (r, c, _, _) = path[i];
        path.push_back((r + dr, c + dc, dr, dc));
    }
    );
    path.pop_front();
    let polygon: Vec<(f64, f64)> = path.iter().map(|(r, c, _dr, _dc)| (*r, *c)).collect();
    let dsa = double_signed_area(&polygon);
    let orientation = (dsa / dsa.abs()) as isize; // +1 clockwise, -1 anti-clockwise
    println!("orientation {orientation}");
    println!("{:?}", polygon);
    
    if orientation > 0 { // clockwise
        for i in 0..path.len() {
            let (r, c, dr, dc) = path[i];
            let next_i = (i + 1) % path.len();
            // println!("this: {:?}", path[i]);
            // println!("next: {:?}", path[next_i]);
            let (_nr, _nc, ndr, ndc) = path[next_i];
            if dc < 0. {
                path[i].0 = r.ceil();
                if ndr > 0. {
                    path[i].1 = c.ceil(); // L + D
                } else {
                    path[i].1 = c.floor(); // L + U
                }
            } else if dc > 0. {
                path[i].0 = r.floor();
                if ndr > 0. {
                    path[i].1 = c.ceil(); // R + D
                } else {
                    path[i].1 = c.floor(); // R + U
                }
            } else if dr > 0. { 
                path[i].1 = c.ceil();
                if ndc > 0. {
                    path[i].0 = r.floor(); // D + R
                } else {
                    path[i].0 = r.ceil(); // D + L
                }
            } else if dr < 0. {
                path[i].1 = c.floor();
                if ndc > 0. {
                    path[i].0 = r.floor(); // U + R
                } else {
                    path[i].0 = r.ceil();  // U + L
                }
            }            
        }
    } else { // anti-clockwise
        for i in 0..path.len() {
            let (r, c, dr, dc) = path[i];
            let next_i = (i + 1) % path.len();
            let (_nr, _nc, ndr, ndc) = path[next_i];
            if dc < 0. {
                path[i].0 = r.floor();
                if ndr > 0. {
                    path[i].1 = c.floor();
                } else {
                    path[i].1 = c.ceil();
                }
            } else if dc > 0. {
                path[i].0 = r.ceil();
                if ndr > 0. {
                    path[i].1 = c.floor();
                } else {
                    path[i].1 = c.ceil();
                }
            } else if dr > 0. { 
                path[i].1 = c.floor();
                if ndc > 0. {
                    path[i].0 = r.ceil();
                } else {
                    path[i].0 = r.floor();
                }
            } else if dr < 0. {
                path[i].1 = c.ceil();
                if ndc > 0. {
                    path[i].0 = r.ceil();
                } else {
                    path[i].0 = r.floor();
                }
            }     
        }
    }

    path.push_front(path[path.len() - 1]);
    
    let polygon: Vec<(f64, f64)> = path.iter().map(|(r, c, _dr, _dc)| (*r, *c)).collect();


    println!("{:?}", polygon);
    let task1 = double_signed_area(&polygon).abs() / 2.;
    println!("Task 1: {}", task1);

}

fn double_signed_area(p: &Vec<(f64, f64)>) -> f64 {
    segments(p)
            .map(|(a, b)| {
                // println!("{:?}", (x0, y0, x1, y1));
                crossprod(&a, &b)
            })
            .sum::<f64>()
}

fn segments(p: &Vec<(f64, f64)>) -> Zip<std::slice::Iter<(f64, f64)>, std::vec::IntoIter<(f64, f64)>> {
    zip(p, p[1..].iter().map(|(x, y)| (x + p[0].0, y + p[0].1)).collect::<Vec<_>>())
}

fn crossprod(x: &(f64, f64), y: &(f64, f64)) -> f64 {
    x.0 * y.1 - x.1 * y.0
}