use std::fs;

const FILE_PATH: &str = "./input/input.txt";

// struct
fn main() { 
    let mut contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    contents = contents.replace("S", "0");

    let mut grid = contents.lines().map(|l| l
        .chars().map(|c| c.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let nrow = grid.len();
    let ncol = grid[0].len();
    // let mut n_steps: usize = 0;
    let step_target = 64;
    let mut tsk1_grid = grid.clone();
    for step in 0..step_target {
        let step_char = step.to_string();
        let mut positions: Vec<(usize, usize)> = vec![];
        for r in 0..nrow {
            for c in 0..ncol {
                if tsk1_grid[r][c] == step_char {positions.push((r, c))}
            }
        }
        for (r, c) in positions {
            let poss_dr = if r == 0 {
                vec![0, 1]
            } else if r == nrow - 1 {
                vec![-1, 0]
            } else {                    
                vec![-1, 0, 1]
            };
            let poss_dc = if c == 0 {
                vec![0, 1]
            } else if c == ncol - 1 {
                vec![-1, 0]
            } else {                    
                vec![-1, 0, 1]
            };
            let nstep_char = (step+1).to_string();
            for dr in poss_dr {
                for dc in poss_dc.clone() {
                    if dr == dc || dr == -dc {continue;}
                    let nr = (r as isize + dr) as usize;
                    let nc = (c as isize + dc) as usize;
                    if tsk1_grid[nr][nc] == "." {
                        tsk1_grid[nr][nc] = nstep_char.clone();
                    }
                }
            }
            
        }
    }
    let gardens = tsk1_grid.iter().flatten().map(|c| c.parse::<usize>());
    let task1 = gardens.filter(|g: &Result<_, _>| g.as_ref().is_ok_and(|x| x % 2 == 0)).count();
    println!("Task 1: {}", task1);

    // --- TASK 2 --- //
    // let step_target = 26501365;
    // for step in 0..step_target {
    //     let step_char = step.to_string();
    //     let mut positions: Vec<(usize, usize)> = vec![];
    //     for r in 0..nrow {
    //         for c in 0..ncol {
    //             if tsk1_grid[r][c] == step_char {positions.push((r, c))}
    //         }
    //     }
    //     for (r, c) in positions {
    //         let poss_dr = if r == 0 {
    //             vec![0, 1]
    //         } else if r == nrow - 1 {
    //             vec![-1, 0]
    //         } else {                    
    //             vec![-1, 0, 1]
    //         };
    //         let poss_dc = if c == 0 {
    //             vec![0, 1]
    //         } else if c == ncol - 1 {
    //             vec![-1, 0]
    //         } else {                    
    //             vec![-1, 0, 1]
    //         };
    //         let nstep_char = (step+1).to_string();
    //         for dr in poss_dr {
    //             for dc in poss_dc.clone() {
    //                 if dr == dc || dr == -dc {continue;}
    //                 let nr = (r as isize + dr) as usize;
    //                 let nc = (c as isize + dc) as usize;
    //                 if tsk1_grid[nr][nc] == "." {
    //                     tsk1_grid[nr][nc] = nstep_char.clone();
    //                 }
    //             }
    //         }
            
    //     }
    // }
}
