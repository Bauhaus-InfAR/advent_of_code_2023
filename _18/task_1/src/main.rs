use std::{fs, iter::{zip, Zip}};
use std::collections::VecDeque;

const FILE_PATH: &str = "../input/input.txt";

fn main() {
    let contents: String = fs::read_to_string(FILE_PATH)
    .expect("File should load");
    let lines: Vec<(char, f32, &str)> = contents.lines().map(|l| {
        let mut split = l.split_ascii_whitespace();
        let dir = split.next().unwrap().chars().nth(0).unwrap();
        let length = split.next().unwrap().parse::<f32>().unwrap();
        let color = split.next().unwrap();
        (dir, length, color)
    }
    ).collect::<Vec<_>>();
    let mut path = VecDeque::from(vec![(0.5, 0.5, 0., 0.)]);
    lines.iter().enumerate().for_each(|(i, (dir, len, _color))| {
        let (dr, dc) = match dir {
            'D' => (*len, 0.),
            'U' => (-*len, 0.),
            'R' => (0., *len),
            'L' => (0., -*len),
            _ => (0., 0.)
        };
        let (r, c, _, _) = path[i];
        path.push_back((r + dr, c + dc, dr, dc));
    }
    );
    path.pop_front();
    let polygon: Vec<(f32, f32)> = path.iter().map(|(r, c, _dr, _dc)| (*r, *c)).collect();
    let dsa = double_signed_area(&polygon);
    let orientation = (dsa / dsa.abs()) as isize; // +1 clockwise, -1 anti-clockwise
    
    if orientation > 0 { // clockwise
        for i in 0..path.len() {
            let (r, c, dr, dc) = path[i];
            let next_i = (i + 1) % path.len();
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
    
    let polygon: Vec<(f32, f32)> = path.iter().map(|(r, c, _dr, _dc)| (*r, *c)).collect();

    let task1 = double_signed_area(&polygon).abs() / 2.;
    println!("Task 1: {}", task1);

}

fn double_signed_area(p: &Vec<(f32, f32)>) -> f32 {
    segments(p)
            .map(|(a, b)| {
                crossprod(&a, &b)
            })
            .sum::<f32>()
}

fn segments(p: &Vec<(f32, f32)>) -> Zip<std::slice::Iter<(f32, f32)>, std::vec::IntoIter<(f32, f32)>> {
    zip(p, p[1..].iter().map(|(x, y)| (x + p[0].0, y + p[0].1)).collect::<Vec<_>>())
}

fn crossprod(x: &(f32, f32), y: &(f32, f32)) -> f32 {
    x.0 * y.1 - x.1 * y.0
}