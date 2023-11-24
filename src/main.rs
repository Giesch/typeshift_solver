use std::time::Instant;

use typeshift_solver::Typeshift;

// TODO
// eliminate all string processing at startup
// give an optimal solution order based on speed of new letters used
// optionally rank all minimal solutions based on best optimal ordering
// add real benchmark testing

fn main() {
    let start = Instant::now();

    let input = include_str!("../files/puzzles/2023-11-19.txt");
    let typeshift = Typeshift::new(input);
    let prep_ts = Instant::now();
    let prep_time = prep_ts.duration_since(start);

    let (solution, steps) = typeshift.find_best_solution();
    let total_time = Instant::now().duration_since(start);
    let solve_time = Instant::now().duration_since(prep_ts);
    let size = typeshift.size();

    dbg!(size, solution, steps, total_time, prep_time, solve_time);
}
