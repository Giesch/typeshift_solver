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
    let columns = typeshift::into_columns(input);
    let words = unsafe { index::load() };
    let after_load = Instant::now();

    let typeshift = Typeshift::new(columns, words);
    dbg!(typeshift.size());
    let (solution, steps) = typeshift.find_best_solution();
    let after_solve = Instant::now();

    dbg!(solution);
    dbg!(steps);

    let load = after_load.duration_since(start);
    let solve = after_solve.duration_since(after_load);
    let total = after_solve.duration_since(start);

    dbg!(load);
    dbg!(solve);
    dbg!(total);
}
