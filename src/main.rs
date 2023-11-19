use std::time::Instant;

use typeshift_solver::Typeshift;

// TODO add real benchmark testing

fn main() {
    let start = Instant::now();

    let input = include_str!("../files/puzzles/2023-11-19.txt");
    let typeshift = Typeshift::new(input);

    let (solution, steps) = typeshift.find_best_solution();
    let time = Instant::now().duration_since(start);
    let size = typeshift.size();

    dbg!(size);
    dbg!(solution);
    dbg!(steps);
    dbg!(time);
}
