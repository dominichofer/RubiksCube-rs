use rand::{RngExt, SeedableRng, rngs::StdRng};
use rubikscube::*;
use std::hint::black_box;
use std::time::Instant;

fn bench<T, F: FnMut(&T)>(name: &str, items: &[T], mut f: F) {
    let start = Instant::now();
    for item in items {
        f(item);
    }
    let ns = start.elapsed().as_nanos() as f64 / items.len() as f64;
    println!("{:<25} {:>8.1} ns", name, ns);
}

fn main() {
    let twister = Twister::new();
    const ITERATIONS: usize = 1_000_000;
    let mut corners = Corners::solved();
    let mut edges = Edges::solved();
    let mut subset_index = SubsetIndex::solved();
    let mut coset_index = CosetIndex::solved();
    let mut cube_index = CubeIndex::solved();
    let mut rnd_twist = RandomTwistGen::new(42, &ALL_TWISTS);
    let mut rnd_subset_twist = RandomTwistGen::new(42, &H0_TWISTS);
    let mut rnd = StdRng::seed_from_u64(42);
    let rnd_twists = rnd_twist.gen_twists(ITERATIONS);
    let rnd_subset_twists = rnd_subset_twist.gen_twists(ITERATIONS);
    let rnd_rotation = Vec::from_iter((0..ITERATIONS).map(|_| if rnd.random_bool(0.5) { Rotation::X } else { Rotation::Y }));
    let rnd_corners = Vec::from_iter((0..ITERATIONS).map(|_| Corners::twists(&rnd_twist.gen_twists(100))));
    let rnd_edges = Vec::from_iter((0..ITERATIONS).map(|_| Edges::twists(&rnd_twist.gen_twists(100))));

    let corners_from_indices: Vec<(usize, usize)> = (0..ITERATIONS)
        .map(|_|
            (
                rnd.random_range(0..Corners::PRM_SIZE),
                rnd.random_range(0..Corners::ORI_SIZE),
            ))
        .collect();
    let edges_from_indices: Vec<(usize, usize, usize, usize, usize, usize, usize)> = (0..ITERATIONS)
        .map(|_|
            (
                rnd.random_range(0..binomial(12, 4)),
                rnd.random_range(0..binomial(12, 4)),
                rnd.random_range(0..binomial(12, 4)),
                rnd.random_range(0..factorial(4)),
                rnd.random_range(0..factorial(4)),
                rnd.random_range(0..factorial(4)),
                rnd.random_range(0..factorial(4)),
            ))
        .collect();
    let edges_from_subset_indices: Vec<(usize, usize)> = (0..ITERATIONS)
        .map(|_|
            (
                rnd.random_range(0..factorial(8)),
                rnd.random_range(0..factorial(4)),
            ))
        .collect();

    bench("Corners twisted", &rnd_twists, |&t| corners = Corners::twist(t) * corners );
    bench("Corners conjugated_by", &rnd_rotation, |&r| corners = corners.conjugated_by(r) );
    bench("Corners from_indices", &corners_from_indices, |&(prm, ori)| corners = Corners::from_indices(prm, ori) );
    bench("Corners prm_index", &rnd_corners, |c| { black_box(c.prm_index()); });
    bench("Corners ori_index", &rnd_corners, |c| { black_box(c.ori_index()); });

    bench("Edges twisted", &rnd_twists, |&t| edges = Edges::twist(t) * edges);
    bench("Edges conjugated_by", &rnd_rotation, |&r| edges = edges.conjugated_by(r) );
    bench(
        "Edges from_indices",
        &edges_from_indices,
        |&(x_loc, y_loc, z_loc, x_prm, y_prm, z_prm, ori)| {
            black_box(Edges::from_indices(LocPrm::new(x_loc, x_prm), LocPrm::new(y_loc, y_prm), LocPrm::new(z_loc, z_prm), ori));
        },
    );
    bench(
        "Edges from_subset_indices",
        &edges_from_subset_indices,
        |&(loc, prm)| {
            black_box(Edges::from_subset_indices(loc, prm));
        },
    );
    bench("Edges x_loc_prm_index", &rnd_edges, |e| {
        black_box(e.x_loc_prm_index());
    });
    bench("Edges y_loc_prm_index", &rnd_edges, |e| {
        black_box(e.y_loc_prm_index());
    });
    bench("Edges z_loc_prm_index", &rnd_edges, |e| {
        black_box(e.z_loc_prm_index());
    });
    bench("Edges xy_prm_index", &rnd_edges, |e| {
        black_box(e.xy_prm_index());
    });
    bench("Edges ori_index", &rnd_edges, |e| {
        black_box(e.ori_index());
    });

    bench("SubsetIndex twisted", &rnd_subset_twists, |&t| {
        subset_index = subset_index.twisted(&twister, t)
    });
    bench("CosetIndex twisted", &rnd_twists, |&t| {
        coset_index = coset_index.twisted(&twister, t)
    });
    bench("CubeIndex twisted", &rnd_twists, |&t| {
        cube_index = cube_index.twisted(&twister, t)
    });

    black_box(corners);
    black_box(edges);
    black_box(subset_index);
    black_box(coset_index);
    black_box(cube_index);
}
