use std::time::Instant;

use typeshift_solver::*;

// TODO
// improve next_words ranking heuristics (rare letters?)
// do real benchmark testing
// use snapshot testing library for unit tests

fn main() {
    let start = Instant::now();

    let input = include_str!("../files/puzzles/2023-11-19.txt");
    let typeshift = Typeshift::new(input);
    dbg!(typeshift.size());

    let (solution, steps) = typeshift.find_best_solution();
    let time = Instant::now().duration_since(start);

    dbg!(solution);
    dbg!(steps);
    dbg!(time);
}
