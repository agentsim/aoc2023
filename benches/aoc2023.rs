use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::*;

pub fn day1(c: &mut Criterion) {
    c.bench_function("aoc 1.1", |b| b.iter(|| aoc1::solve1()));
    c.bench_function("aoc 1.2", |b| b.iter(|| aoc1::solve2()));
}

criterion_group!(benches, day1);
criterion_main!(benches);