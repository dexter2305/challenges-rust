use challenges_rust::leetcode::p_unindexed::{sorted_squares_v1, sorted_squares_v2};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn sorted_squares(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorted_squares");

    for size in [10, 100, 1000, 10_000].iter() {
        let ints: Vec<i32> = (-(*size as i32) / 2..=*size as i32 / 2).collect();
        group.bench_with_input(BenchmarkId::new("v1", size), size, |b, _| {
            b.iter(|| sorted_squares_v1(black_box(ints.clone())))
        });

        group.bench_with_input(BenchmarkId::new("v2", size), size, |b, _| {
            b.iter(|| sorted_squares_v2(black_box(ints.clone())))
        });
    }
    group.finish();
}

criterion_group!(benches, sorted_squares);
criterion_main!(benches);
