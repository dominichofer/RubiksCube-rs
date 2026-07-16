use rubikscube::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_pos_file>", args[0]);
        std::process::exit(1);
    }
    let pos_file_path = &args[1];

    init_twister();
    init_subset_twister();
    init_subset_index();
    pin_process_to_core().unwrap_or_else(|err| eprintln!("Warning: could not pin process to one core: {err}"));
    set_process_priority().unwrap_or_else(|err| eprintln!("Warning: could not raise process priority: {err}"));

    let twist_sequences = read_twist_file(pos_file_path);
    assert!(twist_sequences.len() > 0, "No twist sequences found in the file!");
    let positions = Vec::from_iter(twist_sequences.iter().map(|twists| Cube::solved().twisted_by(twists)));

    let (corners_table, subset_table, coset_table) = get_tables();

    let mut solver = TwoPhaseSolver::new(
        &coset_table,
        &subset_table,
        &corners_table,
    );
        
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
