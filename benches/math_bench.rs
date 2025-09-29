use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustiks_cube::{binomial, combination_index, nth_combination};

fn binomial_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("binomial");

    group.bench_function("12_choose_6", |b| {
        b.iter(|| binomial(black_box(12), black_box(6)))
    });
    
    group.bench_function("13_choose_6", |b| {
        b.iter(|| binomial(black_box(13), black_box(6)))
    });
    
    group.finish();
}

fn combination_index_benchmark(c: &mut Criterion) {
    c.bench_function("combination_index", |b| {
        let combo = vec![2, 5, 7, 9];
        b.iter(|| combination_index(black_box(10), black_box(&combo)))
    });
}

fn nth_combination_benchmark(c: &mut Criterion) {
    c.bench_function("nth_combination", |b| {
        let mut combo = vec![0i64; 4];
        b.iter(|| nth_combination(black_box(10), black_box(4), black_box(100), black_box(&mut combo)))
    });
}

criterion_group!(benches, 
    binomial_benchmark,
    combination_index_benchmark, 
    nth_combination_benchmark
);
criterion_main!(benches);
