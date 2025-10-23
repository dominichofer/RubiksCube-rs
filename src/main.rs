use std::collections::HashMap;
use rustiks_cube::*;

fn corners_distance_table(path: &str) -> DistanceTable {
    if std::fs::File::open(path).is_err() {
        let start = std::time::Instant::now();
        let table = create_distance_table(
            Twists::all(),
            Corners::solved(),
            |c: Corners| c.index() as usize,
            |i: usize| Corners::from_index(0, i as u16),
            Corners::INDEX_SIZE as usize,
        );
        let elapsed = start.elapsed();
        println!("Corners table created in: {:?}", elapsed);

        store_distance_table(path, &table);
    }

    DistanceTable::from_file(path)
}

fn subset_distance_table(path: &str) -> DistanceTable {
    if std::fs::File::open(path).is_err() {
        let start = std::time::Instant::now();
        let table = create_distance_table(
            Twists::h0(),
            SubsetCube::solved(),
            |s: SubsetCube| s.index() as usize,
            |i: usize| SubsetCube::from_index(i as u64),
            SubsetCube::INDEX_SIZE as usize,
        );
        let elapsed = start.elapsed();
        println!("Subset table created in: {:?}", elapsed);
        
        store_distance_table(path, &table);
    }

    DistanceTable::from_file(path)
}

fn coset_distance_table(path: &str) -> DistanceTable {
    if std::fs::File::open(path).is_err() {
        let start = std::time::Instant::now();
        let table = create_distance_table(
            Twists::h0(),
            CosetCube::solved(),
            |c: CosetCube| c.index() as usize,
            |i: usize| CosetCube::from_index(i as u32),
            CosetCube::INDEX_SIZE as usize,
        );
        let elapsed = start.elapsed();
        println!("Coset table created in: {:?}", elapsed);
        
        store_distance_table(path, &table);
    }

    DistanceTable::from_file(path)
}

fn load_config(path: &str) -> HashMap<String, String> {
    let contents = std::fs::read_to_string(path)
        .expect("Failed to read config file");
    
    contents.lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let config = load_config("config.txt");
    
    let corners_path = config.get("corners_table")
        .expect("corners_table not found in config");
    let subset_path = config.get("subset_table")
        .expect("subset_table not found in config");
    let coset_path = config.get("coset_table")
        .expect("coset_table not found in config");
    
    let corners_table = corners_distance_table(corners_path);
    let subset_table = subset_distance_table(subset_path);
    let coset_table = coset_distance_table(coset_path);
}
