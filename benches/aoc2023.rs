use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::*;

pub fn day1(c: &mut Criterion) {
    //c.bench_function("aoc 1.1", |b| b.iter(|| aoc01::solve1()));
    //c.bench_function("aoc 1.2", |b| b.iter(|| aoc01::solve2()));
    //c.bench_function("aoc 2.1", |b| b.iter(|| aoc02::solve1()));
    //c.bench_function("aoc 2.2", |b| b.iter(|| aoc02::solve2()));
    c.bench_function("aoc 3.1", |b| b.iter(|| aoc03::solve1()));
    c.bench_function("aoc 3.2", |b| b.iter(|| aoc03::solve2()));
}

criterion_group!(benches, day1);
criterion_main!(benches);