use rubikscube::*;

/// Parse a string of space-separated twist names into a vector of Twist values.
/// Anything onwards from '#' is ignored.
fn parse_line(input: &str) -> Vec<Twist> {
    input
        .split('#') // Split off comments
        .next() // Take the part before the comment, or the whole line if there is no comment
        .unwrap_or("") // Handle the case where the line is empty or only contains a comment
        .split_whitespace()
        .map(|s| s.parse().unwrap()) // Parse each twist name into a Twist value, panicking if any are invalid
        .collect()
}

fn read_twist_file(path: &str) -> Vec<Vec<Twist>> {
    let content = std::fs::read_to_string(path).unwrap();
    content.lines().map(|line| parse_line(line)).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_pos_file>", args[0]);
        std::process::exit(1);
    }
    let pos_file_path = &args[1];

    let stored_tables = StoredTables::load("config.txt");

    // let mut coset_solver = CosetCover::new(
    //     &twister,
    //     &subset_table,
    //     20,
    // );

    // let mut rnd = RandomTwistGen::new(42, TwistSet::full());
    // let coset_cube = CosetIndex::solved().twisted_by(&twister, &rnd.gen_twists(100));
    // println!("Covering coset with subset distance {}", coset_table.distance(coset_cube.index()));

    // coset_solver.reset_for(coset_cube);

    // let path_iterator = CosetToSubsetPathsIterator::new(
    //     &twister,
    //     &coset_table,
    //     coset_cube,
    // );

    // let mut counter = 0;
    // let start = std::time::Instant::now();
    // for twists in path_iterator {
    //     counter += 1;
    //     if counter % 100000 == 0 {
    //         println!("Generated {} paths in {:?}. speed: {:.2} paths/sec", counter, start.elapsed(), counter as f64 / start.elapsed().as_secs_f64());
    //     }
    //     // coset_solver.cover_with(&twists);
    // }

    let mut solver = TwoPhaseSolver::new(
        &stored_tables.twister,
        &stored_tables.coset,
        &stored_tables.subset,
        &stored_tables.corners,
    );

    let mut total_time = std::time::Duration::ZERO;
    let twist_sequences = read_twist_file(pos_file_path);

    for (i, twists) in twist_sequences.iter().enumerate() {
        let cube = CubeIndex::solved().twisted_by(&stored_tables.twister, twists);

        let start = std::time::Instant::now();
        let solution = solver.solve(cube, 20).unwrap();
        let elapsed = start.elapsed();
        total_time += elapsed;

        // Verify solution
        if cube.twisted_by(&stored_tables.twister, &solution) != CubeIndex::solved() {
            panic!(
                "Incorrect solution found on line {}! Solution: {:?}",
                i + 1,
                solution
            );
        }
    }

    println!("Total time taken: {:?}", total_time);
    if twist_sequences.len() > 0 {
        println!(
            "Average time per solve: {:?}",
            total_time / twist_sequences.len() as u32
        );
    }
    solver.print_stats();
}
