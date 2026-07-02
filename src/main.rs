use rubikscube::*;

#[cfg(target_os = "windows")]
fn pin_to_core() -> std::io::Result<()> {
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, SetProcessAffinityMask};

    unsafe {
        let process: HANDLE = GetCurrentProcess();

        if SetProcessAffinityMask(process, 1) == 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn pin_to_core() -> std::io::Result<()> {
    unsafe {
        let mut cpu_set = std::mem::zeroed::<libc::cpu_set_t>();
        libc::CPU_ZERO(&mut cpu_set);
        libc::CPU_SET(0, &mut cpu_set);

        if libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &cpu_set) != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn set_priority() -> std::io::Result<()> {
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, HIGH_PRIORITY_CLASS, SetPriorityClass};

    unsafe {
        let process: HANDLE = GetCurrentProcess();

        if SetPriorityClass(process, HIGH_PRIORITY_CLASS) == 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn set_priority() -> std::io::Result<()> {
    unsafe {
        // -10 requests higher priority; may require additional privileges.
        if libc::setpriority(libc::PRIO_PROCESS, 0, -10) != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn pin_to_core() -> std::io::Result<()> {
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn set_priority() -> std::io::Result<()> {
    Ok(())
}

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
    if let Err(err) = pin_to_core() {
        eprintln!("Warning: could not pin process to one core: {err}");
    }
    if let Err(err) = set_priority() {
        eprintln!("Warning: could not raise process priority: {err}");
    }

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
