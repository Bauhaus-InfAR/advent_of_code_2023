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

    let mut visited: Vec<(isize, isize)> = vec![];
    let mut crossroads: Vec<(isize, isize)> = vec![];
    let mut path_steps: Vec<usize> = vec![];

    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        let step = state.step;
        let (r, c) = state.position;
        let (dr, dc) = state.delta;

        // if step % 1000 == 0 {
        //     println!("{step}");
        // }
        if visited.contains(&(r, c)) || (r == nrows && c == ncols - 1) {
            if crossroads.len() != 0 {
                
            println!("{:?}", visited);
                let last_crossroad = crossroads.pop().unwrap();
                // visited.reverse();
                let pos = visited.iter().position(|p| p == &last_crossroad).unwrap();

                let slice = &visited[..pos+1];
                visited = slice.to_vec();
            println!("step {step}; pos {pos}");
            println!("{:?}", visited);
            println!("");
                // visited.reverse();
            }
            if r == nrows && c == ncols - 1 {
                path_steps.push(step);
                // println!("{}", path_steps.iter().max().unwrap());
            println!("{:?}", visited);
            }
            continue;
        };

        if visited.contains(&(r, c)) {
            let last_crossroad = crossroads.pop().unwrap();
            let pos = visited.iter().position(|p| p == &last_crossroad).unwrap();
            let slice = &visited[..pos+1];
            visited = slice.to_vec();
            continue;
        };
        visited.push((r, c));

        let mut possible_moves = vec![];
        for (ndr, ndc) in ALL_MOVES {
            if (dr, dc) == (-ndr, -ndc) { continue; }
            let nr = r + ndr;
            let nc = c + ndc;
            
            if nr < 0 || nr > nrows || nc < 0 || nc > ncols {
                continue;
            }            
            if grid[nr as usize][nc as usize] == '#' { continue;}
            possible_moves.push((ndr, ndc));
            if possible_moves.len() > 1 {
                crossroads.push((r, c));
            }
            heap.push(State {
                position: (nr, nc),
                delta: (ndr, ndc),
                distance: nrows - nr + ncols - nc,
                step: step + 1
            })

        }
    }

    let task2 = path_steps.iter().max().unwrap();
    println!("Task 2: {}", task2);
}