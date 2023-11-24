use criterion::{black_box, criterion_group, criterion_main, Criterion};

use typeshift_solver::Typeshift;

fn bench_typeshift(input: &str) {
    let typeshift = Typeshift::new(input);
    let _ = typeshift.find_best_solution();
}

fn criterion_benchmark(c: &mut Criterion) {
    let nov_19 = include_str!("../files/puzzles/2023-11-19.txt");
    c.bench_function("Slowest Puzzle (Nov 19)", |b| {
        b.iter(|| bench_typeshift(black_box(nov_19)))
    });

    let nov_16 = include_str!("../files/puzzles/2023-11-16.txt");
    c.bench_function("Fast Puzzle (Nov 16)", |b| {
        b.iter(|| bench_typeshift(black_box(nov_16)))
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
