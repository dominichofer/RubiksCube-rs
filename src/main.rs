use std::collections::HashMap;
use rubikscube::*;

fn parse_twist_sequence(sequence: &str) -> Vec<Twist> {
    sequence.split_whitespace()
        .filter_map(|s| s.parse::<Twist>().ok())
        .collect()
}

fn parse_twist_sequences_from_file(path: &str) -> Result<Vec<Vec<Twist>>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    let sequences = content.lines()
        .map(|line| parse_twist_sequence(line))
        .collect();
    Ok(sequences)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_cube_file>", args[0]);
        std::process::exit(1);
    }
    let cube_file_path = &args[1];

    let config: HashMap<String, String> = std::fs::read_to_string("config.txt").unwrap()
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect();

    let corners_table_path = config.get("corners_table").unwrap();
    let subset_table_path = config.get("subset_table").unwrap();
    let coset_table_path = config.get("coset_table").unwrap();

    let twister = Twister::new();
    let corners_table = corners_distance_table(&twister, corners_table_path);
    let subset_table = subset_distance_table(&twister, subset_table_path);
    let coset_table = coset_direction_table(&twister, coset_table_path);

    let mut solver = TwoPhaseSolver::new(
        twister.clone(),
        coset_table,
        subset_table,
        corners_table,
    );

    let mut total_time = std::time::Duration::ZERO;
    let twist_sequences = parse_twist_sequences_from_file(cube_file_path).unwrap();
    
    for (i, twists) in twist_sequences.iter().enumerate() {
        let cube = Cube::solved().twisted_by(&twister, twists);

        let start = std::time::Instant::now();
        let solution = solver.solve(cube.subset, cube.coset, 20).unwrap();
        let elapsed = start.elapsed();
        total_time += elapsed;
        
        // Verify solution
        if !cube.twisted_by(&twister, &solution).is_solved() {
            panic!("Incorrect solution found on line {}! Solution: {:?}", i + 1, solution);
        }
        //println!("Solved scramble {} in: {:?}. Solution: {:?}", i + 1, elapsed, solution);
    }
    
    println!("Total time taken: {:?}", total_time);
    if twist_sequences.len() > 0 {
        println!("Average time per solve: {:?}", total_time / twist_sequences.len() as u32);
    }
    solver.print_stats();
}
