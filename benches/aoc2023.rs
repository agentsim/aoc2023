use criterion::{criterion_group, criterion_main, Criterion};
use aoc::*;

pub fn day_01(c: &mut Criterion) {
    c.bench_function("aoc 1.1", |b| b.iter(|| aoc01::solve1()));
    c.bench_function("aoc 1.2", |b| b.iter(|| aoc01::solve2()));
}

pub fn day_02(c: &mut Criterion) {
    c.bench_function("aoc 2.1", |b| b.iter(|| aoc02::solve1()));
    c.bench_function("aoc 2.2", |b| b.iter(|| aoc02::solve2()));
}

pub fn day_03(c: &mut Criterion) {
    c.bench_function("aoc 3.1", |b| b.iter(|| aoc03::solve1()));
    c.bench_function("aoc 3.2", |b| b.iter(|| aoc03::solve2()));
}

pub fn day_04(c: &mut Criterion) {
    c.bench_function("aoc 4.1", |b| b.iter(|| aoc04::solve1()));
    c.bench_function("aoc 4.2", |b| b.iter(|| aoc04::solve2()));
}

pub fn day_05(c: &mut Criterion) {
    c.bench_function("aoc 5.1", |b| b.iter(|| aoc05::solve1()));
    c.bench_function("aoc 5.2", |b| b.iter(|| aoc05::solve2()));
}

pub fn day_06(c: &mut Criterion) {
    c.bench_function("aoc 6.1", |b| b.iter(|| aoc06::solve1()));
    c.bench_function("aoc 6.2", |b| b.iter(|| aoc06::solve2()));
}

pub fn day_07(c: &mut Criterion) {
    c.bench_function("aoc 7.1", |b| b.iter(|| aoc07::solve1()));
    c.bench_function("aoc 7.2", |b| b.iter(|| aoc07::solve2()));
}

criterion_group!(
    name = aoc;
    config = Criterion::default();
    targets = day_01, day_02, day_03, day_04, day_05, day_06, day_07);
criterion_main!(aoc);