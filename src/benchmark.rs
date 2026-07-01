use rand::{RngExt, SeedableRng, rngs::StdRng};
use rubikscube::*;
use std::hint::black_box;
use std::time::Instant;

struct Benchmarker {
    stored_tables: StoredTables,
    iterations: usize,
    rnd: StdRng,
    rnd_twist: Vec<Twist>,
    rnd_twists: Vec<Vec<Twist>>,
    rnd_subset_twist: Vec<Twist>,
    rnd_rotation: Vec<Axis>,
    rnd_cube: Vec<Cube>,
    rnd_subset_cube: Vec<SubsetCube>,
}

impl Benchmarker {
    fn new(iterations: usize) -> Self {
        let stored_tables = StoredTables::load("config.txt");
        let mut rnd = StdRng::seed_from_u64(42);
        let mut rnd_twist_gen = RandomTwistGen::new(42, &ALL_TWISTS);
        let mut rnd_subset_twist_gen = RandomTwistGen::new(42, &H0_TWISTS);
        let rnd_twist = rnd_twist_gen.gen_twists(iterations);
        let rnd_twists: Vec<Vec<Twist>> = (0..iterations).map(|_| rnd_twist_gen.gen_twists(100)).collect();
        let rnd_subset_twist = rnd_subset_twist_gen.gen_twists(iterations);
        let rnd_subset_twists: Vec<Vec<Twist>> = (0..iterations).map(|_| rnd_subset_twist_gen.gen_twists(100)).collect();
        let rnd_rotation = (0..iterations).map(|_| if rnd.random_bool(0.5) { Axis::X } else { Axis::Y }).collect();
        let rnd_cube = rnd_twists.iter().map(|t| Cube::solved().twisted_by(t)).collect();
        let rnd_subset_cube = rnd_subset_twists.iter().map(|t| SubsetCube::solved().twisted_by(t)).collect();
        Self {
            stored_tables,
            iterations,
            rnd,
            rnd_twist,
            rnd_twists,
            rnd_subset_twist,
            rnd_rotation,
            rnd_cube,
            rnd_subset_cube,
        }
    }

    // Generate a test vector of random values within a given range
    fn test_vec_of_random_range<R>(&mut self, range: R) -> Vec<usize>
    where
        R: rand::distr::uniform::SampleRange<usize> + Clone,
    {
        (0..self.iterations)
            .map(|_| self.rnd.random_range(range.clone()))
            .collect()
    }

    fn test_vec_of_twists<R, F: Fn(&Vec<Twist>) -> R>(&mut self, function: F) -> Vec<R>
    {
        self.rnd_twists.iter().map(|twists| function(twists)).collect()
    }
    
    fn bench<T, R, F: FnMut(&T) -> R>(&self, name: &str, items: &[T], mut function: F) {
        let start = Instant::now();
        for item in items {
            black_box(function(item));
        }
        let ns = start.elapsed().as_nanos() as f64 / items.len() as f64;
        println!("{:<25} {:>8.1} ns", name, ns);
    }
    
    fn bench_nth_permutation(&mut self) {
        let rnd_factorial_4 = self.test_vec_of_random_range(0..factorial(4));
        let rnd_factorial_8 = self.test_vec_of_random_range(0..factorial(8));
        self.bench("nth_permutation (len 4)", &rnd_factorial_4, |&i| Permutation::<4>::from_index(i));
        self.bench("nth_permutation (len 8)", &rnd_factorial_8, |&i| Permutation::<8>::from_index(i));
    }
    
    fn bench_permutation_index(&mut self) {
        let rnd_factorial_4 = self.test_vec_of_random_range(0..factorial(4));
        let rnd_factorial_8 = self.test_vec_of_random_range(0..factorial(8));
        let rnd_permutation_4: Vec<Permutation<4>> = rnd_factorial_4.iter().map(|&i| Permutation::<4>::from_index(i)).collect();
        let rnd_permutation_8: Vec<Permutation<8>> = rnd_factorial_8.iter().map(|&i| Permutation::<8>::from_index(i)).collect();
        self.bench("permutation_index (len 4)", &rnd_permutation_4, |p| { p.index() });
        self.bench("permutation_index (len 8)", &rnd_permutation_8, |p| { p.index() });
    }
    
    fn bench_nth_combination(&mut self) {
        let rnd_binomial_12_4 = self.test_vec_of_random_range(0..binomial(12, 4));
        let mut out = [0usize; 4];
        self.bench("nth_combination (12, 4)", &rnd_binomial_12_4, |&i| { nth_combination(12, 4, i) });
        self.bench("nth_combination2 (12, 4)", &rnd_binomial_12_4, |&i| { nth_combination2(12, i, &mut out) });
    }

    fn bench_encode(&mut self) {
        let rnd_base2: Vec<Vec<usize>> = (0..self.iterations).map(|_| (0..11).map(|_| self.rnd.random_range(0..2)).collect()).collect();
        let rnd_base3: Vec<Vec<usize>> = (0..self.iterations).map(|_| (0..8).map(|_| self.rnd.random_range(0..3)).collect()).collect();
        self.bench("encode (base 2)", &rnd_base2, |v| { encode(&v, 2) });
        self.bench("encode (base 3)", &rnd_base3, |v| { encode(&v, 3) });
    }
    
    fn bench_decode(&mut self) {
        let rnd_encoded_base2 = self.test_vec_of_random_range(0..2_usize.pow(11));
        let rnd_encoded_base3 = self.test_vec_of_random_range(0..3_usize.pow(7));
        self.bench("decode (base 2)", &rnd_encoded_base2, |&n| { decode(n, 2, 11) });
        self.bench("decode (base 3)", &rnd_encoded_base3, |&n| { decode(n, 3, 7) });
    }

    fn bench_corners(&mut self) {
        let rnd_corners = self.test_vec_of_twists(|t| Corners::twists(t));
        let corners_from_indices: Vec<(usize, usize)> = Vec::from_iter((0..self.iterations).map(|_|(self.rnd.random_range(0..Corners::PRM_SIZE), self.rnd.random_range(0..Corners::ORI_SIZE))));
        let mut corners = Corners::solved();
        self.bench("Corners twist", &self.rnd_twist, |&t| corners = t * corners );
        self.bench("Corners conjugated_by", &self.rnd_rotation, |&r| corners = corners.conjugated_by(r) );
        self.bench("Corners from_indices", &corners_from_indices, |&(prm, ori)| corners = Corners::from_indices(prm, ori) );
        self.bench("Corners prm_index", &rnd_corners, |c| { c.prm_index() });
        self.bench("Corners ori_index", &rnd_corners, |c| { c.ori_index() });
        black_box(corners);
    }

    fn bench_edges(&mut self) {
        let rnd_edges = self.test_vec_of_twists(|t| Edges::twists(t));
        let edges_from_indices: Vec<(usize, usize, usize, usize, usize, usize, usize)> = (0..self.iterations)
            .map(|_|
                (
                    self.rnd.random_range(0..binomial(12, 4)),
                    self.rnd.random_range(0..binomial(12, 4)),
                    self.rnd.random_range(0..binomial(12, 4)),
                    self.rnd.random_range(0..factorial(4)),
                    self.rnd.random_range(0..factorial(4)),
                    self.rnd.random_range(0..factorial(4)),
                    self.rnd.random_range(0..factorial(4)),
                ))
            .collect();
        let edges_from_subset_indices: Vec<(usize, usize)> = (0..self.iterations)
            .map(|_|
                (
                    self.rnd.random_range(0..factorial(8)),
                    self.rnd.random_range(0..factorial(4)),
                ))
            .collect();
        let mut edges = Edges::solved();
    
        self.bench("Edges twist", &self.rnd_twist, |&t| edges = t * edges);
        self.bench("Edges conjugated_by", &self.rnd_rotation, |&r| edges = edges.conjugated_by(r) );
        self.bench(
            "Edges from_indices",
            &edges_from_indices,
            |&(x_loc, y_loc, z_loc, x_prm, y_prm, z_prm, ori)| {
                Edges::from_indices(LocPrm::new(x_loc, x_prm), LocPrm::new(y_loc, y_prm), LocPrm::new(z_loc, z_prm), ori)
            },
        );
        self.bench(
            "Edges from_subset_indices",
            &edges_from_subset_indices,
            |&(loc, prm)| {
                Edges::from_subset_indices(loc, prm)
            },
        );
        self.bench("Edges x_loc_prm_index", &rnd_edges, |e| { e.loc_prm(Axis::X) });
        self.bench("Edges y_loc_prm_index", &rnd_edges, |e| { e.loc_prm(Axis::Y) });
        self.bench("Edges z_loc_prm_index", &rnd_edges, |e| { e.loc_prm(Axis::Z) });
        self.bench("Edges xy_prm_index", &rnd_edges, |e| { e.xy_prm_index() });
        self.bench("Edges ori_index", &rnd_edges, |e| { e.ori_index() });
        black_box(edges);
    }

    fn bench_subset_cube(&mut self) {
        let subset_cube_index = self.test_vec_of_random_range(0..SubsetCube::INDEX_SIZE);
        let mut subset_cube = SubsetCube::solved();
        self.bench("SubsetCube twisted", &self.rnd_subset_twist, |&t| { subset_cube = subset_cube.twisted(t) });
        self.bench("SubsetCube from_index", &subset_cube_index, |&i| { SubsetCube::from_index(i) });
        self.bench("SubsetCube index", &self.rnd_subset_cube, |c| { c.index() });
        black_box(subset_cube);
    }
    
    fn bench_cube(&mut self) {
        let cube_corner_index = self.test_vec_of_random_range(0..Cube::CORNER_INDEX_SIZE);
        let cube_coset_index = self.test_vec_of_random_range(0..Cube::COSETS_INDEX_SIZE);
        let mut cube = Cube::solved();
    
        self.bench("Cube twisted", &self.rnd_twist, |&t| { cube = cube.twisted(t) });
        self.bench("Cube from_corner_index", &cube_corner_index, |&i| { Cube::from_corner_index(i) });
        self.bench("Cube from_coset_index", &cube_coset_index, |&i| { Cube::from_coset_index(i) });
        self.bench("Cube corner_index", &self.rnd_cube, |c| { c.corner_index() });
        self.bench("Cube subset_cube", &self.rnd_cube, |c| { c.subset_cube() });
        self.bench("Cube coset_index", &self.rnd_cube, |c| { c.coset_index() });
        black_box(cube);
    }
    
    fn bench_distances(&mut self) {
        self.bench("Corners distance", &self.rnd_cube, |c| { self.stored_tables.corners.distance(c.corner_index()) });
        self.bench("Coset distance", &self.rnd_cube, |c| { self.stored_tables.coset.distance(c.coset_index()) });
        self.bench("Subset distance", &self.rnd_subset_cube, |c| { self.stored_tables.subset.distance(c.index()) });
    }

    fn bench_phase_2(&mut self) {
        let mut solver = TwoPhaseSolver::new(
            &self.stored_tables.coset,
            &self.stored_tables.subset,
            &self.stored_tables.corners
        );
        let foo = self.rnd_subset_cube.iter().map(|&c| (c, self.stored_tables.subset.distance(c.index()))).collect::<Vec<_>>();
        self.bench("TwoPhaseSolver phase_2", &foo, |&c| { solver.search_phase_2(c.0, c.1) });
    }
}

fn main() {
    init_twister();
    let mut benchmarker = Benchmarker::new(10_000_000);
    benchmarker.bench_nth_permutation();
    benchmarker.bench_nth_combination();
    benchmarker.bench_permutation_index();
    benchmarker.bench_encode();
    benchmarker.bench_decode();
    benchmarker.bench_corners();
    benchmarker.bench_edges();
    benchmarker.bench_subset_cube();
    benchmarker.bench_cube();
    benchmarker.bench_distances();
    benchmarker.bench_phase_2();
}
