use std::collections::BTreeSet;

use typeshift_solver::*;

use insta::{assert_yaml_snapshot, glob, with_settings};
use serde::Serialize;

#[derive(Serialize)]
struct SolutionSnapshot {
    size: usize,
    steps: usize,
    solution: BTreeSet<&'static str>,
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
        let (solution, steps) = typeshift.find_best_solution();
        let size = typeshift.size();

        let info = SnapshotInfo::new(&input);

        with_settings!({
            description => name,
            info => &info
        }, {
            assert_yaml_snapshot!(SolutionSnapshot {
                size,
                steps,
                solution,
            });
        });
    });
}
