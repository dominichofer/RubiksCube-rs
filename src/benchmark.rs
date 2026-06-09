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
    init_twister();
    
    const ITERATIONS: usize = 1_000_000;
    let mut rnd = StdRng::seed_from_u64(42);

    // nth_permutation
    let rnd_factorial_4: Vec<usize> = (0..ITERATIONS).map(|_| rnd.random_range(0..factorial(4))).collect();
    let rnd_factorial_8: Vec<usize> = (0..ITERATIONS).map(|_| rnd.random_range(0..factorial(8))).collect();
    bench("nth_permutation (len 4)", &rnd_factorial_4, |&i| { black_box(Permutation::<4>::from_index(i)); });
    bench("nth_permutation (len 8)", &rnd_factorial_8, |&i| { black_box(Permutation::<8>::from_index(i)); });

    // permutation_index
    let rnd_permutation_4: Vec<Permutation<4>> = (0..ITERATIONS).map(|_| Permutation::<4>::from_index(rnd.random_range(0..factorial(4)))).collect();
    let rnd_permutation_8: Vec<Permutation<8>> = (0..ITERATIONS).map(|_| Permutation::<8>::from_index(rnd.random_range(0..factorial(8)))).collect();
    bench("permutation_index (len 4)", &rnd_permutation_4, |p| { black_box(p.index()); });
    bench("permutation_index (len 8)", &rnd_permutation_8, |p| { black_box(p.index()); });

    // nth_combination
    let rnd_binomial_12_4: Vec<usize> = (0..ITERATIONS).map(|_| rnd.random_range(0..binomial(12, 4))).collect();
    bench("nth_combination (12, 4)", &rnd_binomial_12_4, |&i| { black_box(nth_combination(12, 4, i)); });
    let mut out = [0usize; 4];
    bench("nth_combination2 (12, 4)", &rnd_binomial_12_4, |&i| { black_box(nth_combination2(12, i, &mut out)); });

    // encode
    let rnd_binary: Vec<Vec<usize>> = (0..ITERATIONS).map(|_| (0..11).map(|_| rnd.random_range(0..2)).collect()).collect();
    let rnd_base3: Vec<Vec<usize>> = (0..ITERATIONS).map(|_| (0..8).map(|_| rnd.random_range(0..3)).collect()).collect();
    bench("encode (base 2)", &rnd_binary, |v| { black_box(encode(&v, 2)); });
    bench("encode (base 3)", &rnd_base3, |v| { black_box(encode(&v, 3)); });

    // decode
    let rnd_encoded_binary: Vec<usize> = (0..ITERATIONS).map(|_| rnd.random_range(0..(1 << 11))).collect();
    let rnd_encoded_base3: Vec<usize> = (0..ITERATIONS).map(|_| rnd.random_range(0..(3_usize.pow(8)))).collect();
    bench("decode (base 2)", &rnd_encoded_binary, |&n| { black_box(decode(n, 2, 11)); });
    bench("decode (base 3)", &rnd_encoded_base3, |&n| { black_box(decode(n, 3, 8)); });

    let mut corners = Corners::solved();
    let mut edges = Edges::solved();
    let mut subset_index = SubsetIndex::solved();
    let mut cube_index = Cube::solved();

    let mut rnd_twist = RandomTwistGen::new(42, &ALL_TWISTS);
    let mut rnd_subset_twist = RandomTwistGen::new(42, &H0_TWISTS);
    let rnd_twists = rnd_twist.gen_twists(ITERATIONS);
    let rnd_subset_twists = rnd_subset_twist.gen_twists(ITERATIONS);
    let rnd_rotation = Vec::from_iter((0..ITERATIONS).map(|_| if rnd.random_bool(0.5) { Axis::X } else { Axis::Y }));
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


    bench("Corners twist", &rnd_twists, |&t| corners = t * corners );
    bench("Corners conjugated_by", &rnd_rotation, |&r| corners = corners.conjugated_by(r) );
    bench("Corners from_indices", &corners_from_indices, |&(prm, ori)| corners = Corners::from_indices(prm, ori) );
    bench("Corners prm_index", &rnd_corners, |c| { black_box(c.prm_index()); });
    bench("Corners ori_index", &rnd_corners, |c| { black_box(c.ori_index()); });

    bench("Edges twist", &rnd_twists, |&t| edges = t * edges);
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
    bench("Edges x_loc_prm_index", &rnd_edges, |e| { black_box(e.loc_prm(Axis::X)); });
    bench("Edges y_loc_prm_index", &rnd_edges, |e| { black_box(e.loc_prm(Axis::Y)); });
    bench("Edges z_loc_prm_index", &rnd_edges, |e| { black_box(e.loc_prm(Axis::Z)); });
    bench("Edges xy_prm_index", &rnd_edges, |e| { black_box(e.xy_prm_index()); });
    bench("Edges ori_index", &rnd_edges, |e| { black_box(e.ori_index()); });

    bench("SubsetIndex twisted", &rnd_subset_twists, |&t| { subset_index = subset_index.twisted(t) });
    bench("Cube twisted", &rnd_twists, |&t| { cube_index = cube_index.twisted(t) });

    let rnd_cubes= Vec::from_iter((0..ITERATIONS).map(|_| Cube::solved().twisted_by(&rnd_twist.gen_twists(100))));
    bench("Cube corner_index", &rnd_cubes, |c| { black_box(c.corner_index()); });
    bench("Cube subset_index", &rnd_cubes, |c| { black_box(c.subset_index()); });
    bench("Cube coset_index", &rnd_cubes, |c| { black_box(c.coset_index()); });

    black_box(corners);
    black_box(edges);
    black_box(subset_index);
    black_box(cube_index);
}
