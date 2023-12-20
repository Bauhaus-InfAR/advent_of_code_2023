use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    heat_loss: isize,
    position: (isize, isize),
    delta: (isize, isize),
    consec_moves: isize,
    distance: isize
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.heat_loss.cmp(&self.heat_loss).then_with(|| other.distance.cmp(&self.distance))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const FILE_PATH: &str = "./input/input.txt";
const ALL_MOVES: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let contents: String = fs::read_to_string(FILE_PATH)
    .expect("File should load");
    let grid: Vec<Vec<u32>> = contents.lines().map(|l| l
        .chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
    ).collect::<Vec<_>>();

    let nrows = grid.len() as isize - 1;
    let ncols = grid[0].len() as isize - 1;

    let mut heap = BinaryHeap::new();

    heap.push(State {
        heat_loss: 0,
        position: (0, 0),
        delta: (0, 0),
        consec_moves: 0,
        distance: nrows + ncols
    });
    
    // visited = (r, c, dr, dc, consec)
    let mut visited: Vec<(isize, isize, isize, isize, isize)> = vec![];
    let mut task1: isize = 0;
    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        let (r, c) = state.position;
        let (dr, dc) = state.delta;

        if r == nrows && c == ncols {            
            println!("{:?}", state);
            task1 = state.heat_loss;
            break;
        }

        if visited.contains(&(r, c, dr, dc, state.consec_moves)) {
            continue;
        }

        visited.push((r, c, dr, dc, state.consec_moves));

        if state.consec_moves < 3 && state.delta != (0, 0) {
            let nr = r + dr;
            let nc = c + dc;
            if nr < 0 || nr > nrows || nc < 0 || nc > ncols {
                continue;
            }
            let nhl = grid[nr as usize][nc as usize] as isize;
            heap.push(State {
                heat_loss: state.heat_loss + nhl,
                position: (nr, nc),
                delta: (dr, dc),
                consec_moves: state.consec_moves + 1,
                distance: nrows - nr + ncols - nc
            })
        }

        for (ndr, ndc) in ALL_MOVES {
            if (dr, dc) == (-ndr, -ndc) || (dr, dc) == (ndr, ndc)  { continue; }
            let nr = r + ndr;
            let nc = c + ndc;
            if nr < 0 || nr > nrows || nc < 0 || nc > ncols {
                continue;
            }
            let nhl = grid[nr as usize][nc as usize] as isize;
            heap.push(State {
                heat_loss: state.heat_loss + nhl,
                position: (nr, nc),
                delta: (ndr, ndc),
                consec_moves: 1,
                distance: nrows - nr + ncols - nc
            })

        }
    }
    println!("Task 1: {}", task1);
}