use criterion::{criterion_group, criterion_main, Criterion};
use rubikscube::{Corners, Edges, SubsetCube, CosetCube, Cube, RandomTwistGen, Twistable, TwistSet};
use std::hint::black_box;

fn corners_twist_benchmark(c: &mut Criterion) {
    c.bench_function("corners_twist", |b| {
        let mut rnd = RandomTwistGen::new(42, TwistSet::full());
        let mut cube = Corners::solved();
        b.iter(|| {
            let twist = rnd.gen_twist();
            cube = cube.twisted(twist);
            black_box(cube);
        })
    });
}

fn edges_twist_benchmark(c: &mut Criterion) {
    c.bench_function("edges_twist", |b| {
        let mut rnd = RandomTwistGen::new(42, TwistSet::full());
        let mut cube = Edges::solved();
        b.iter(|| {
            let twist = rnd.gen_twist();
            cube = cube.twisted(twist);
            black_box(cube);
        })
    });
}

fn subset_cube_twist_benchmark(c: &mut Criterion) {
    c.bench_function("subset_cube_twist", |b| {
        let mut rnd = RandomTwistGen::new(42, TwistSet::full());
        let mut cube = SubsetCube::solved();
        b.iter(|| {
            let twist = rnd.gen_twist();
            cube = cube.twisted(twist);
            black_box(cube);
        })
    });
}

fn coset_cube_twist_benchmark(c: &mut Criterion) {
    c.bench_function("coset_cube_twist", |b| {
        let mut rnd = RandomTwistGen::new(42, TwistSet::full());
        let mut cube = CosetCube::solved();
        b.iter(|| {
            let twist = rnd.gen_twist();
            cube = cube.twisted(twist);
            black_box(cube);
        })
    });
}

fn cube_twist_benchmark(c: &mut Criterion) {
    c.bench_function("cube_twist", |b| {
        let mut rnd = RandomTwistGen::new(42, TwistSet::full());
        let mut cube = Cube::solved();
        b.iter(|| {
            let twist = rnd.gen_twist();
            cube = cube.twisted(twist);
            black_box(cube);
        })
    });
}

criterion_group!(benches,
    corners_twist_benchmark,
    edges_twist_benchmark,
    subset_cube_twist_benchmark,
    coset_cube_twist_benchmark,
    cube_twist_benchmark
);
criterion_main!(benches);
