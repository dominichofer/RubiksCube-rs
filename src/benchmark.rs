use rubikscube::{Rotation, Corners, CosetCube, Edges, RandomTwistGen, SubsetCube, TwistSet, Twistable, Twister};
use std::hint::black_box;
use std::time::Instant;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

fn bench<T, F: FnMut(&T)>(name: &str, items: &[T], mut f: F) {
    let start = Instant::now();
    for item in items {
        f(item);
    }
    let ns = start.elapsed().as_nanos() as f64 / items.len() as f64;
    println!("{:<25} {:>8.1} ns", name, ns);
}

fn main() {
    const ITERATIONS: usize = 1_000_000;
    let twister = Twister::new();
    let mut corners = Corners::solved();
    let mut edges = Edges::solved();
    let mut subset_cube = SubsetCube::solved();
    let mut coset_cube = CosetCube::solved();
    let mut rnd_twist = RandomTwistGen::new(42, TwistSet::full());
    let mut rnd_subset_twist = RandomTwistGen::new(42, TwistSet::h0());
    let mut rnd = StdRng::seed_from_u64(42);
    let rnd_twists = Vec::from_iter((0..ITERATIONS).map(|_| rnd_twist.gen_twist()));
    let rnd_subset_twists = Vec::from_iter((0..ITERATIONS).map(|_| rnd_subset_twist.gen_twist()));
    let rnd_rotation = Vec::from_iter((0..ITERATIONS).map(|_| if rnd.random_bool(0.5) { Rotation::L } else { Rotation::U }));
    let rnd_corners = Vec::from_iter((0..ITERATIONS).map(|_| Corners::solved().twisted_by(&rnd_twist.gen_twists(100))));
    let rnd_edges = Vec::from_iter((0..ITERATIONS).map(|_| Edges::solved().twisted_by(&rnd_twist.gen_twists(100))));

    let corners_from_indices: Vec<(usize, usize)> = (0..ITERATIONS)
        .map(|_| (rnd.random_range(0..Corners::PRM_SIZE), rnd.random_range(0..Corners::ORI_SIZE)))
        .collect();
    let edges_from_indices: Vec<(usize, usize, usize, usize)> = (0..ITERATIONS)
        .map(|_| (
            rnd.random_range(0..Edges::SLICE_PRM_SIZE),
            rnd.random_range(0..Edges::NON_SLICE_PRM_SIZE),
            rnd.random_range(0..Edges::SLICE_LOC_SIZE),
            rnd.random_range(0..Edges::ORI_SIZE),
        ))
        .collect();

    bench("Corners twisted", &rnd_twists, |&t| corners = corners.twisted(t));
    bench("Corners rotated_colours", &rnd_rotation, |&r| corners = corners.rotated_colours(r));
    bench("Corners from_indices", &corners_from_indices, |&(prm, ori)| corners = Corners::from_indices(prm, ori));
    bench("Corners prm_index", &rnd_corners, |c| { black_box(c.prm_index()); });
    bench("Corners ori_index", &rnd_corners, |c| { black_box(c.ori_index()); });
    
    bench("Edges twisted", &rnd_twists, |&t| edges = edges.twisted(t));
    bench("Edges rotated_colours", &rnd_rotation, |&r| edges = edges.rotated_colours(r));
    bench("Edges from_indices", &edges_from_indices, |&(sp, nsp, sl, ori)| { black_box(Edges::from_indices(sp, nsp, sl, ori)); });
    bench("Edges slice_prm_index", &rnd_edges, |e| { black_box(e.slice_prm_index()); });
    bench("Edges non_slice_prm_index", &rnd_edges, |e| { black_box(e.non_slice_prm_index()); });
    bench("Edges slice_loc_index", &rnd_edges, |e| { black_box(e.slice_loc_index()); });
    bench("Edges ori_index", &rnd_edges, |e| { black_box(e.ori_index()); });

    bench("SubsetCube twisted", &rnd_subset_twists, |&t| subset_cube = subset_cube.twisted(&twister, t));

    bench("CosetCube twisted", &rnd_twists, |&t| coset_cube = coset_cube.twisted(&twister, t));

    println!();
    black_box(corners);
    black_box(edges);
    black_box(subset_cube);
    black_box(coset_cube);
}
