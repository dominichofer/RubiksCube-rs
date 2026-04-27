use crate::cubies::*;
use crate::index::*;
use crate::table::*;
use std::collections::HashMap;

fn read_config(path: &str) -> HashMap<String, String> {
    let content = std::fs::read_to_string(path).unwrap();
    content
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            (key, value)
        })
        .collect()
}

pub struct StoredTables {
    pub twister: Twister,
    pub corners: DistanceTable,
    pub subset: DistanceTable,
    pub coset: DirectionsTable,
}

impl StoredTables {
    pub fn load(config_path: &str) -> Self {
        let config = read_config(config_path);
        let twister = Twister::new();
        let corners_table = corners_distance_table(&twister, &config["corners_table"]);
        let subset_table = subset_distance_table(&twister, &config["subset_table"]);
        let coset_table = coset_direction_table(&twister, &config["coset_table"]);
        Self {
            twister,
            corners: corners_table,
            subset: subset_table,
            coset: coset_table,
        }
    }
}

fn corners_distance_table(twister: &Twister, path: &str) -> DistanceTable {
    let time = std::time::Instant::now();
    let result = DistanceTable::from_file(path);
    let table: DistanceTable;
    if result.is_ok() {
        println!("Corners table loaded in: {:?}", time.elapsed());
        table = result.unwrap();
    } else {
        table = DistanceTable::create(
            twister,
            &ALL_TWISTS,
            CornerIndex::solved(),
            |c: CornerIndex| c.index(),
            |i: usize| CornerIndex::from_index(i),
            CornerIndex::INDEX_SIZE,
        );
        println!("Corners table created in: {:?}", time.elapsed());

        table.save_to_file(path).unwrap();
    }

    // Verify data integrity
    let mut counts = vec![0; 12];
    for i in 0..CornerIndex::INDEX_SIZE {
        let d = table.distance(i);
        counts[d as usize] += 1;
    }
    // According to https://oeis.org/A080629
    assert_eq!(
        counts,
        vec![
            1, 18, 243, 2874, 28000, 205416, 1168516, 5402628, 20776176, 45391616, 15139616, 64736
        ]
    );

    table
}

fn subset_distance_table(twister: &Twister, path: &str) -> DistanceTable {
    let time = std::time::Instant::now();
    let result = DistanceTable::from_file(path);
    let table: DistanceTable;
    if result.is_ok() {
        println!("Subset table loaded in: {:?}", time.elapsed());
        table = result.unwrap();
    } else {
        table = DistanceTable::create(
            twister,
            &H0_TWISTS,
            SubsetIndex::solved(),
            |s: SubsetIndex| s.index(),
            |i: usize| SubsetIndex::from_index(i),
            SubsetIndex::INDEX_SIZE,
        );
        println!("Subset table created in: {:?}", time.elapsed());

        table.save_to_file(path).unwrap();
    }

    // Verify data integrity
    let mut counts = vec![0u64; 19];
    for i in 0..SubsetIndex::INDEX_SIZE {
        let d = table.distance(i);
        counts[d as usize] += 1;
    }
    assert_eq!(
        counts,
        vec![
            1, 10, 67, 456, 3079, 19948, 123074, 736850, 4185118, 22630733, 116767872, 552538680,
            2176344160, 5627785188, 7172925794, 3608731814, 224058996, 1575608, 1352
        ]
    );

    table
}

fn coset_direction_table(twister: &Twister, path: &str) -> DirectionsTable {
    let time = std::time::Instant::now();
    let result = DirectionsTable::from_file(path);
    let table: DirectionsTable;
    if result.is_ok() {
        println!("Coset table loaded in: {:?}", time.elapsed());
        table = result.unwrap();
    } else {
        table = DirectionsTable::create(
            twister,
            &ALL_TWISTS,
            CosetIndex::solved(),
            |c: CosetIndex| c.index(),
            |i: usize| CosetIndex::from_index(i),
            CosetIndex::INDEX_SIZE,
        );
        println!("Coset table created in: {:?}", time.elapsed());

        table.save_to_file(path).unwrap();
    }

    // Verify data integrity
    let mut counts = vec![0u64; 13];
    for i in 0..CosetIndex::INDEX_SIZE {
        let d = table.distance(i);
        counts[d as usize] += 1;
    }
    assert_eq!(
        counts,
        vec![
            1, 4, 50, 592, 7156, 87236, 1043817, 12070278, 124946368, 821605960, 1199128738,
            58202444, 476
        ]
    );

    table
}
