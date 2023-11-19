use typeshift_solver::*;

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
        let (_solution, steps) = typeshift.find_best_solution();

        println!("{name}\n  size:  {size}\n  steps: {steps}");
    }
}
