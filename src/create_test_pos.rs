use rubikscube::*;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <twist_sequences> <file>", args[0]);
        std::process::exit(1);
    }
    let twist_sequences: usize = args[1].parse().expect("Failed to parse twist sequences");
    let file: &str = &args[2];

    let stored_tables = StoredTables::load("config.txt");

    let mut rnd_twist = RandomTwistGen::new(42, &ALL_TWISTS);
    let cubes = Vec::from_iter((0..twist_sequences)
        .map(|_| CubeIndex::solved().twisted_by(&stored_tables.twister, &rnd_twist.gen_twists(100))));

    let out = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file)
        .expect("Failed to open output file");
    let out = Mutex::new(out);

    cubes.par_iter().for_each(|&cube| {
        let mut solver = TwoPhaseSolver::new(
            &stored_tables.twister,
            &stored_tables.coset,
            &stored_tables.subset,
            &stored_tables.corners,
        );
        let solution = solver.solve(cube, 20).unwrap();
        assert!(cube.twisted_by(&stored_tables.twister, &solution) == CubeIndex::solved(), "Incorrect solution found! Solution: {:?}", solution);
        let line = solution.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>().join(" ");
        let mut out = out.lock().unwrap();
        writeln!(out, "{}", line).expect("Failed to write solution");
    });
}
