#[macro_use]
extern crate criterion;

use std::time::Duration;

use criterion::Criterion;

extern crate aoc_2018;

fn day1(c: &mut Criterion) {
    c.bench_function("day1", |b| b.iter(|| aoc_2018::days::day1()));
}

fn day2(c: &mut Criterion) {
    c.bench_function("day2", |b| b.iter(|| aoc_2018::days::day2()));
}

fn day3(c: &mut Criterion) {
    c.bench_function("day3", |b| b.iter(|| aoc_2018::days::day3()));
}

fn day4(c: &mut Criterion) {
    c.bench_function("day4", |b| b.iter(|| aoc_2018::days::day4()));
}
fn day5(c: &mut Criterion) {
    c.bench_function("day5", |b| b.iter(|| aoc_2018::days::day5()));
}
fn day6(c: &mut Criterion) {
    c.bench_function("day6", |b| b.iter(|| aoc_2018::days::day6()));
}
fn day7(c: &mut Criterion) {
    c.bench_function("day7", |b| b.iter(|| aoc_2018::days::day7()));
}
fn day8(c: &mut Criterion) {
    c.bench_function("day8", |b| b.iter(|| aoc_2018::days::day8()));
}
fn day9(c: &mut Criterion) {
    c.bench_function("day9", |b| b.iter(|| aoc_2018::days::day9()));
}
fn day10(c: &mut Criterion) {
    c.bench_function("day10", |b| b.iter(|| aoc_2018::days::day10()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(15).measurement_time(Duration::new(10, 0));
    targets = day1, day2, day3, day4, day5, day6, day7, day8, day9, day10
}
criterion_main!(benches);
