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
    init_twister();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_pos_file>", args[0]);
        std::process::exit(1);
    }
    let pos_file_path = &args[1];

    let stored_tables = StoredTables::load("config.txt");

    let mut solver = TwoPhaseSolver::new(
        &stored_tables.coset,
        &stored_tables.subset,
        &stored_tables.corners,
    );

    let twist_sequences = read_twist_file(pos_file_path);
    assert!(twist_sequences.len() > 0, "No twist sequences found in the file!");
    let positions = Vec::from_iter(twist_sequences.iter().map(|twists| Cube::solved().twisted_by(twists)));
    
    let mut total_time = std::time::Duration::ZERO;
    for (i, cube) in positions.iter().enumerate() {
        let start = std::time::Instant::now();
        let solution = solver.solve(*cube, 20).unwrap();
        let elapsed = start.elapsed();
        total_time += elapsed;

        // Verify solution
        assert!(cube.twisted_by(&solution) == Cube::solved(), "Incorrect solution found on line {}! Solution: {:?}", i + 1, solution);
    }

    println!("Total time taken: {:?}", total_time);
    println!("Average time per solve: {:?}", total_time / twist_sequences.len() as u32);
    solver.print_stats();
}
