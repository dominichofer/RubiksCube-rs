use rubikscube::*;

fn main() {
    let (corners_table, subset_table, coset_table) = load_tables("config.txt");
    let mut solver = TwoPhaseSolver::new(
        coset_table,
        subset_table,
        corners_table,
    );

    let mut total_time = std::time::Duration::ZERO;
    let mut rnd = RandomTwistGen::new(181086, Twists::all());
    for i in 0..1000 {
        let mut cube = Cube::solved();
        let twists = rnd.gen_twists(20);
        for &twist in &twists {
            cube = cube.twisted(twist);
        }
        let start = std::time::Instant::now();
        let solution = solver.solve(cube, 20);
        let elapsed = start.elapsed();
        total_time += elapsed;

        if solution.is_err() {
            panic!("{}", solution.err().unwrap());
        }
        let solution = solution.unwrap();
        for &twist in &solution {
            cube = cube.twisted(twist);
        }
        if cube != Cube::solved() {
            panic!("Incorrect solution found! {:?} {:?}", solution, twists);
        }
        println!("Solved scramble {} in: {:?}. Solution: {:?}", i, elapsed, solution);
    }
    println!("Total time taken: {:?}", total_time);
    println!("Average time per solve: {:?}", total_time / 100);
    solver.print_stats();
}
