use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = "./input/input.txt";

fn main() {    
    let valid_neighbours: HashMap<(isize, isize), [char; 3]> =  HashMap::from([
        ((0, -1), ['|', 'F', '7']),
        ((-1, 0), ['-', 'F', 'L']),
        ((0, 1), ['|', 'L', 'J']),
        ((1, 0), ['-', 'J', '7'])
    ]);
    

    let moves: HashMap<char, [(isize, isize); 2]> =  HashMap::from([
        ('|', [(0, -1), (0, 1)]),
        ('-', [(1, 0), (-1, 0)]),
        ('F', [(1, 0), (0, 1)]),
        ('7', [(-1, 0), (0, 1)]),
        ('J', [(-1, 0), (0, -1)]),
        ('L', [(1, 0), (0, -1)])
    ]);


    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let lines = contents.lines().collect::<Vec<_>>();
    let line_length = lines[0].len() + if contents.contains('\r') {2} else {1}; // windows \n\r
    let mut output = contents
        .lines()
        .map(|l| l
            .chars()
            .map(|_c| ' ')
            .collect::<Vec<_>>())
            .collect::<Vec<_>>();
    
    let start = contents.find('S').unwrap();
    let mut prev_coord = ((start % line_length) as isize, (start / line_length) as isize);
    let mut current_coord = prev_coord;
    output[current_coord.1 as usize][current_coord.0 as usize] = 'x';
    let width = output[0].len();
    let height = output.len();
    let mut next_step = '.';
    for move_to in valid_neighbours.keys() {
        let tmp_coord = (current_coord.0 + move_to.0, current_coord.1 + move_to.1);
        let tmp_next = find_node(tmp_coord, &lines);
        if valid_neighbours[move_to].contains(&tmp_next) {
            let next_move = get_next_move(tmp_coord, prev_coord, moves[&tmp_next]);
            current_coord = (current_coord.0 + next_move.0, current_coord.1 + next_move.1);
            next_step = find_node(current_coord, &lines);
            output[current_coord.1 as usize][current_coord.0 as usize] = next_step;
            break;
        }
    }
    let mut n_steps = 1;
    while next_step != 'S' {
        n_steps += 1;
        let next_move = get_next_move(current_coord, prev_coord, moves[&next_step]);
        let next_coord = (current_coord.0 + next_move.0, current_coord.1 + next_move.1);      
        next_step = find_node(next_coord, &lines);
        prev_coord = current_coord;
        current_coord = next_coord;
        output[current_coord.1 as usize][current_coord.0 as usize] = next_step;
    }
    let task1 =  (n_steps + 1) / 2;
    let mut task2 = 0;
    let mut prev = 'o';
    for y in 0..height {
        let mut outer = true;
        for x in 0..width {
            if ['|', '-', 'F', 'L', 'J', '7'].contains(&output[y][x]) {
                 outer = match output[y][x] {
                     '|' => !outer,
                     '7' => if prev == 'L' {!outer} else {outer},
                     'J' => if prev == 'F' {!outer} else {outer},
                     _ => outer
                    };
                if !['|', '-'].contains(&output[y][x]) {
                    prev = output[y][x];
                }
                continue;
            }
        if outer {
            output[y][x] = 'o';
        } else {
            output[y][x] = 'i';
            task2 += 1;
        };
        }
    }
    println!("Task 1: {task1}");
    println!("Task 2: {task2}");
}

fn find_node<'a>((x, y): (isize, isize), data: &'a Vec<&str>) -> char {
    let x = x as usize;
    let y = y as usize;
    let line = match data.get(y..y+1) {
        Some(l) => l[0],
        None => return '.'
    };
    let out = match line.chars().nth(x) {
        Some(c) => c,
        None => '.'
    };
    out
}

fn get_next_move(current_coord: (isize, isize), prev_coord: (isize, isize), possible_moves: [(isize, isize); 2]) -> (isize, isize) {
    let tmp: (isize, isize) = (current_coord.0 + possible_moves[0].0, current_coord.1 + possible_moves[0].1);
    if tmp.0 != prev_coord.0 || tmp.1 != prev_coord.1 {
        return possible_moves[0];
    }
    possible_moves[1]
}
