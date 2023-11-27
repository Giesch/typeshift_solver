use typeshift_solver::*;

/// A helper for looking at backtracking performance over all snapshots
fn main() {
    let dir = std::fs::read_dir("./files/puzzles").unwrap();

    let mut puzzles = Vec::new();
    for entry in dir {
        let path = entry.unwrap().path();
        let input = std::fs::read_to_string(&path).unwrap();
        let file_name = path.file_stem().unwrap().to_string_lossy().to_string();

        puzzles.push((file_name, input));
    }

    for (name, input) in puzzles {
        let typeshift = Typeshift::new(&input);
        let size = typeshift.size();
        let (_first_solution, steps) = typeshift.find_first_solution();
        let (all_solutions, _all_steps) = typeshift.find_all_solutions();
        let total_solutions = all_solutions.len();

        println!("{name}\n  size: {size}\n  steps: {steps}\n  solutions: {total_solutions}");
    }
}
