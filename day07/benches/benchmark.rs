use std::fs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day07::both;


pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    c.bench_function("Part 1", |b| b.iter(|| both(black_box(&input), 0)));
    c.bench_function("Part 2", |b| b.iter(|| both(black_box(&input), 1)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);