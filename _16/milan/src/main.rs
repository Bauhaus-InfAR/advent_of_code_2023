use std::{fs, isize, vec};

const FILE_PATH: &str = "./input/input.txt";

fn main() { 
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let grid = contents.lines().map(|l| l
        .chars().collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    let mut paths: Vec<(char, [isize; 2])> = vec![];
    trace(&[0, 0], &grid, 'e', &mut paths);
    let mut task1 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if paths.iter().any(|p| p.1 == [i as isize, j as isize]) {
                task1 += 1;
            }
        }
    }    
    println!("Task 1: {}", task1);
}

fn trace(coord: &[isize; 2], data: &Vec<Vec<char>>, mut direction: char, path: &mut Vec<(char, [isize; 2])>) -> () {
    let mut x = coord[0].clone();
    let mut y = coord[1].clone();
    
    let height = data.len() as isize;
    let width = data[0].len() as isize;
    
    loop {
        if !(x >= 0 && x < width && y >= 0 && y < height) {
            break;}
        let this_step = (direction, [x, y]);
        if path.contains(&this_step) {break}
        path.push(this_step);

        let current_char = data[x as usize][y as usize];
        match current_char {
            '-' => match direction {
                    'n' | 's' => {
                        trace(&[x.clone(), y.clone() + 1], data, 'e', path);
                        trace(&[x.clone(), y.clone() - 1], data, 'w', path);
                        break;
                    },
                    _ => {}
                },
            '|' => match direction {
                'e' | 'w' => {
                    trace(&[x.clone() + 1, y.clone()], data, 's', path);
                    trace(&[x.clone() - 1, y.clone()], data, 'n', path);
                    break;

                },
                _ => {}
            },
            '/' => {
                direction = match direction {
                    'n' => {'e'},
                    'e' => {'n'},
                    's' => {'w'},
                    _ => {'s'}
                }
            },
            '\\' => {
                direction = match direction {
                    'n' => {'w'},
                    'e' => {'s'},
                    's' => {'e'},
                    _ => {'n'}
                }
            },
            _ => {}
        };
        [x, y] = match direction {
            'n' => [x - 1, y],
            'e' => [x, y + 1],
            's' => [x + 1, y],
            _ => [x, y - 1]
        };
    }
}
