use std::time::Instant;

use typeshift_solver::*;

// TODO
// improve next_words ranking heuristics (rare letters?)
// do real benchmark testing; maybe switch to snapshots for unit tests
// try preserving & updating ranking state/index instead of reranking
// rank partial solutions in a smarter way

fn main() {
    let start = Instant::now();

    let input = include_str!("../files/puzzle-11-17-2023.txt");
    let typeshift = Typeshift::new(input);
    dbg!(typeshift.size());

    let (solution, steps) = typeshift.find_best_solution();
    let time = Instant::now().duration_since(start);

    dbg!(solution);
    dbg!(steps);
    dbg!(time);
}
