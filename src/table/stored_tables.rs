use crate::cubies::*;
use crate::index::*;
use crate::table::*;
use crate::table::config_file::read_config_file;

pub fn get_tables() -> (DistanceTable, DistanceTable, DirectionsTable) {
    let config = read_config_file("config.txt");

    let corners_table = DistanceTable::from_file(&config["corners_table"]).unwrap_or_else(|_| create_corners_table());
    let subset_table = DistanceTable::from_file(&config["subset_table"]).unwrap_or_else(|_| create_subset_table());
    let coset_table = DirectionsTable::from_file(&config["coset_table"]).unwrap_or_else(|_| create_coset_table());

    check_corners_table(&corners_table);
    check_subset_table(&subset_table);
    check_coset_table(&coset_table);
    
    (corners_table, subset_table, coset_table)
}

pub fn create_corners_table() -> DistanceTable {
    DistanceTable::create(
        &ALL_TWISTS,
        Cube::solved(),
        |c: Cube| c.corner_index(),
        |i: usize| Cube::from_corner_index(i),
        Cube::CORNER_INDEX_SIZE,
    )
}

pub fn check_corners_table(table: &DistanceTable) {
    // Verify data integrity
    let mut counts = vec![0; 12];
    for i in 0..Cube::CORNER_INDEX_SIZE {
        counts[table.distance(i) as usize] += 1;
    }
    // According to https://oeis.org/A080629
    assert_eq!(counts, vec![1, 18, 243, 2874, 28000, 205416, 1168516, 5402628, 20776176, 45391616, 15139616, 64736]);
}

pub fn create_subset_table() -> DistanceTable {
    DistanceTable::create(
        &H0_TWISTS,
        SubsetCube::solved(),
        |s: SubsetCube| s.index(),
        |i: usize| SubsetCube::from_index(i),
        SubsetCube::INDEX_SIZE,
    )
}

pub fn check_subset_table(table: &DistanceTable) {
    // Verify data integrity
    let mut counts = vec![0u64; 19];
    for i in 0..SubsetCube::INDEX_SIZE {
        counts[table.distance(i) as usize] += 1;
    }
    assert_eq!(
        counts,
        vec![
            1, 10, 67, 456, 3079, 19948, 123074, 736850, 4185118, 22630733, 116767872, 552538680,
            2176344160, 5627785188, 7172925794, 3608731814, 224058996, 1575608, 1352
        ]
    );
}

pub fn create_coset_table() -> DirectionsTable {
    DirectionsTable::create(
        &ALL_TWISTS,
        Cube::solved(),
        |c: Cube| c.coset_index(),
        |i: usize| Cube::from_coset_index(i),
        Cube::COSETS_INDEX_SIZE,
    )
}

pub fn check_coset_table(table: &DirectionsTable) {
    // Verify data integrity
    let mut counts = vec![0u64; 13];
    for i in 0..Cube::COSETS_INDEX_SIZE {
        counts[table.distance(i) as usize] += 1;
    }
    assert_eq!(counts, vec![1, 4, 50, 592, 7156, 87236, 1043817, 12070278, 124946368, 821605960, 1199128738, 58202444, 476]);
}
