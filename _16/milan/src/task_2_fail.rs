use std::{fs, isize, vec};

const FILE_PATH: &str = "./input/input.txt";


fn main() { 
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let grid = contents.lines().map(|l| l
        .chars().collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut all_paths: Vec<Vec<(usize, (char, [isize; 2]))>> = vec![];
    let mut all_n_energised: Vec<usize> = vec![];
    for i in 0..nrows {
        let j_options = if i == 0 || i == nrows-1 {
            (0..ncols).collect::<Vec<_>>()
        } else {
            vec![0, ncols-1]
        };
        for j in j_options {
            let mut possible_start_direction = vec![];
            if j == ncols - 1 {
                possible_start_direction.push('w')
            } else if j == 0 {
                possible_start_direction.push('e')
            }
            if i == 0 {
                possible_start_direction.push('s')
            } else if i == nrows - 1 {
                possible_start_direction.push('n')
            }
            for d in possible_start_direction {
                // println!("start: [{i},{j}]; direction: {d}");
                let mut paths: Vec<(usize, (char, [isize; 2]))> = vec![];
                trace(&[i as isize, j as isize], &grid, d, &mut paths, &all_paths);
                let energised_tiles = paths.iter().map(|p| p.1.1).collect::<Vec<_>>();
                all_n_energised.push(count_energised_tiles(&energised_tiles, nrows, ncols));
                all_paths.push(paths.clone());
                // if i == 0 && j == 5 && d =='s' {
                //     print_grid(&energised_tiles, nrows, ncols);
                //     println!("{:?}", paths);
                //     return;
                // }

            }
            
        }
    }
    
    println!("Task 1: {:?}", all_n_energised[0]);
    let task2 = all_n_energised.iter().max().unwrap();
    println!("Task 2: {}", task2);

}

fn trace(coord: &[isize; 2], data: &Vec<Vec<char>>, mut direction: char, path: &mut Vec<(usize, (char, [isize; 2]))>, all_paths: &Vec< Vec<(usize, (char, [isize; 2]))>>) -> () {
    let mut x = coord[0].clone();
    let mut y = coord[1].clone();
    
    let height = data.len() as isize;
    let width = data[0].len() as isize;
    
    loop {

        if !(x >= 0 && x < width && y >= 0 && y < height) {
            break;}
        let this_step = (path.len(), (direction, [x, y]));
        if path.iter().any(|p| p.1 == this_step.1) {break}
        
        
        path.push(this_step);

        if this_step.0 != 0 {
            let current_char = data[x as usize][y as usize];
            match current_char {
                '-' => match direction {
                        'n' | 's' => {
                            trace(&[x.clone(), y.clone() + 1], data, 'e', path, all_paths);
                            trace(&[x.clone(), y.clone() - 1], data, 'w', path, all_paths);
                            break;
                        },
                        _ => {}
                    },
                '|' => match direction {
                    'e' | 'w' => {
                        trace(&[x.clone() + 1, y.clone()], data, 's', path, all_paths);
                        trace(&[x.clone() - 1, y.clone()], data, 'n', path, all_paths);
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
        }
        [x, y] = match direction {
            'n' => [x - 1, y],
            'e' => [x, y + 1],
            's' => [x + 1, y],
            _ => [x, y - 1]
        };
        let next_step = (direction, [x, y]);
        
        let mut found_match= false;
        
        let mut copy_path_no: usize = 0; 
        let first_matched_step = all_paths.iter().enumerate().map(|(path_no, pths)| {
            match pths.iter().find(|p| p.1 == this_step.1) {
                Some(x) => {
                    // let step_number = x.0;
                    copy_path_no = path_no; 
                    // println!("path to copy {:?}:", copy_path);
                    // copy_path[step_number..].iter().for_each(|x| path.push(*x));
                    // path.append(&mut copy_path[step_number..].to_vec());
                    // println!("step: {}; path: {:?}", path.len(), path);
                    found_match = true;
                    x.0
                },
                None => {usize::MAX}
            }
        }).min();
        if found_match {
            // println!("path to copy {:?}:", copy_path);
            let copy_path = all_paths[copy_path_no].clone();
            // println!("{:?}", copy_path);
            copy_path[first_matched_step.unwrap()..].iter().for_each(|x| path.push(*x));
            // path.append(&mut copy_path[step_number..].to_vec());
            // path.push(this_step);
            // break;
        }
    }
}


fn count_energised_tiles(tiles: &Vec<[isize;2]>, height: usize, width: usize) -> usize {
    let mut out = 0;
    for i in 0..height {
        for j in 0..width {
            if tiles.contains(&[i as isize, j as isize]) {
                out += 1;
            }
        }
    }
    out
}

fn print_grid(tiles: &Vec<[isize;2]>, height: usize, width: usize) {
    for i in 0..height {
        for j in 0..width {
            if tiles.contains(&[i as isize, j as isize]) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("")
    }
println!("")
}