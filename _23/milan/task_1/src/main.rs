use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    position: (isize, isize),
    delta: (isize, isize),
    distance: isize,
    step: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.distance.cmp(&other.distance)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const FILE_PATH: &str = "../input/input.txt";
const ALL_MOVES: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let contents: String = fs::read_to_string(FILE_PATH)
    .expect("File should load");
    let grid: Vec<Vec<char>> = contents.lines().map(|l| l
        .chars().collect::<Vec<char>>()
    ).collect::<Vec<_>>();

    let nrows = grid.len() as isize - 1;
    let ncols = grid[0].len() as isize - 1;

    let mut heap = BinaryHeap::new();

    heap.push(State {
        position: (0, 1),
        delta: (0, 0),
        distance: nrows + ncols - 1,
        step: 0
    });
    
    // visited = (r, c)
    let mut path_steps: Vec<usize> = vec![];
    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        let step = state.step;
        let (r, c) = state.position;
        let (dr, dc) = state.delta;

        if r == nrows && c == ncols - 1 {
            path_steps.push(step)
        }

        let current_field = grid[r as usize][c as usize];
        let mut possible_moves = ALL_MOVES.to_vec();
        if current_field == '<' {
            possible_moves = vec![(0, -1)]
        } else if current_field == '>' {
            possible_moves = vec![(0, 1)]            
        } else if current_field == '^' {
            possible_moves = vec![(-1, 0)]
        } else if current_field == 'v' {
            possible_moves = vec![(1, 0)]            
        }
        for (ndr, ndc) in possible_moves {
            if (dr, dc) == (-ndr, -ndc) { continue; }
            let nr = r + ndr;
            let nc = c + ndc;
            
            if nr < 0 || nr > nrows || nc < 0 || nc > ncols {
                continue;
            }
            
            if grid[nr as usize][nc as usize] == '#' { continue;}
            heap.push(State {
                position: (nr, nc),
                delta: (ndr, ndc),
                distance: nrows - nr + ncols - nc,
                step: step + 1
            })

        }
    }
    let task1 = path_steps.iter().max().unwrap();
    println!("Task 1: {}", task1);
}