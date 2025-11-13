use std::collections::HashMap;
use crate::corners::*;
use crate::tables::*;
use crate::cube::*;
use crate::twist::*;

fn corners_distance_table(path: &str) -> DistanceTable {
    if std::fs::File::open(path).is_err() {
        let start = std::time::Instant::now();
        let table = DistanceTable::create(
            Twists::all(),
            Corners::solved(),
            |c: Corners| c.index() as usize,
            |i: usize| Corners::from_combined_index(i as u32),
            Corners::INDEX_SIZE as usize,
        );
        let elapsed = start.elapsed();
        println!("Corners table created in: {:?}", elapsed);

        table.to_file(path);
    }

    let start = std::time::Instant::now();
    let table = DistanceTable::from_file(path);
    println!("Corners table loaded in: {:?}", start.elapsed());

    // Verify data integrity
    let mut counts = vec![0u64; table.max_distance() as usize + 1];
    for i in 0..table.len() {
        let d = table.distance(i) as usize;
        counts[d] += 1;
    }
    // According to https://oeis.org/A080629
    assert_eq!(counts, vec![
        1, 18, 243, 2_874, 28_000, 205_416, 1_168_516, 5_402_628,
        20_776_176, 45_391_616, 15_139_616, 64_736
    ]);
    assert_eq!(table.max_distance(), 11);

    table
}

fn subset_distance_table(path: &str) -> DistanceTable {
    // if std::fs::File::open(path).is_err() {
    //     let start = std::time::Instant::now();
    //     let table = DistanceTable::create(
    //         Twists::h0(),
    //         SubsetCube::solved(),
    //         |s: SubsetCube| s.index() as usize,
    //         |i: usize| SubsetCube::from_index(i as u64),
    //         SubsetCube::INDEX_SIZE as usize,
    //     );
    //     let elapsed = start.elapsed();
    //     println!("Subset table created in: {:?}", elapsed);
        
    //     table.to_file(path);
    // }

    let start = std::time::Instant::now();
    let table = DistanceTable::from_file(path);
    println!("Subset table loaded in: {:?}", start.elapsed());

    // Verify data integrity
    let mut counts = vec![0u64; table.max_distance() as usize + 1];
    for i in 0..table.len() {
        let d = table.distance(i) as usize;
        counts[d] += 1;
    }
    assert_eq!(counts, vec![
        1, 10, 67, 456, 3079, 19948, 123074, 736850, 4185118, 22630733,
        116767872, 552538680, 2176344160, 5627785188, 7172925794, 3608731814,
        224058996, 1575608, 1352
    ]);
    assert_eq!(table.max_distance(), 18);

    table
}

fn coset_direction_table(path: &str) -> DirectionsTable {
    if std::fs::File::open(path).is_err() {
        let start = std::time::Instant::now();
        let table = DirectionsTable::create(
            Twists::all(),
            CosetCube::solved(),
            |c: CosetCube| c.index() as usize,
            |i: usize| CosetCube::from_index(i as u32),
            CosetCube::INDEX_SIZE as usize,
        );
        let elapsed = start.elapsed();
        println!("Coset table created in: {:?}", elapsed);
        
        table.to_file(path);
    }

    let start = std::time::Instant::now();
    let table = DirectionsTable::from_file(path);
    println!("Coset table loaded in: {:?}", start.elapsed());

    // Verify data integrity
    let mut counts = vec![0u64; table.max_distance() as usize + 1];
    for i in 0..table.len() {
        let d = table.distance(i) as usize;
        counts[d] += 1;
    }
    assert_eq!(counts, vec![
        1, 4, 50, 592, 7156, 87236, 1043817, 12070278, 124946368, 821605960, 1199128738, 58202444, 476
    ]);
    assert_eq!(table.max_distance(), 12);

    table
}

pub fn load_tables(path: &str) -> (DistanceTable, DistanceTable, DirectionsTable) {
    let mut config_file = std::fs::read_to_string(path);
    if config_file.is_err() {
        let default_config = "corners_table=corners_table.dat\nsubset_table=subset_table.dat\ncoset_table=coset_table.dat\n";
        std::fs::write(path, default_config).expect("Unable to write default config file");
        config_file = std::fs::read_to_string(path);
    }
    
    let config: HashMap<String, String> = config_file.unwrap()
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
    
    let corners_path = config.get("corners_table")
        .expect("corners_table not found in config");
    let subset_path = config.get("subset_table")
        .expect("subset_table not found in config");
    let coset_path = config.get("coset_table")
        .expect("coset_table not found in config");
    
    let corners_table = corners_distance_table(corners_path);
    let coset_table = coset_direction_table(coset_path);
    let subset_table = subset_distance_table(subset_path);

    return (corners_table, subset_table, coset_table);
}
