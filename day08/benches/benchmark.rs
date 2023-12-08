use std::fs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day08::{part1, part2};


pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    c.bench_function("Part 1", |b| b.iter(|| part1(black_box(&input))));
    c.bench_function("Part 2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);