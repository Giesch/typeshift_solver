use std::collections::BTreeSet;

use typeshift_solver::*;

use insta::{assert_yaml_snapshot, glob, with_settings};
use serde::Serialize;

#[derive(Serialize)]
struct SolutionSnapshot {
    possible_words: usize,
    steps_to_first_solution: usize,
    first_solution: BTreeSet<&'static str>,
    possible_solutions: usize,
}

#[derive(Serialize)]
struct SnapshotInfo<'a> {
    columns: Vec<&'a str>,
}

impl<'a> SnapshotInfo<'a> {
    fn new(input: &'a str) -> Self {
        let columns = input.lines().collect();
        Self { columns }
    }
}

#[test]
fn puzzle_snapshots() {
    glob!("../files", "puzzles/*.txt", |path| {
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let input = std::fs::read_to_string(path).unwrap();

        let typeshift = Typeshift::new(&input);
        let possible_words = typeshift.size();
        let (first_solution, steps_to_first_solution) = typeshift.find_first_solution();

        let (all_solutions, _all_steps) = typeshift.find_all_solutions();
        let possible_solutions = all_solutions.len();

        let info = SnapshotInfo::new(&input);
        let snapshot = SolutionSnapshot {
            possible_words,
            steps_to_first_solution,
            first_solution,
            possible_solutions,
        };

        with_settings!({ description => name, info => &info }, {
            assert_yaml_snapshot!(snapshot);
        });
    });
}
