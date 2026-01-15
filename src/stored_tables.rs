use crate::tables::{DistanceTable, DirectionsTable};
use crate::cube::{CornersCube, SubsetCube, CosetCube};
use crate::twist::TwistSet;
use crate::twister::Twister;

pub fn corners_distance_table(twister: &Twister, path: &str) -> DistanceTable {
    let time = std::time::Instant::now();
    let result = DistanceTable::from_file(path);
    let table: DistanceTable;
    if result.is_ok() {
        println!("Corners table loaded in: {:?}", time.elapsed());
        table = result.unwrap();
    }
    else {
        table = DistanceTable::create(
            twister,
            TwistSet::full(),
            CornersCube::solved(),
            |c: CornersCube| c.index(),
            |i: usize| CornersCube::from_index(i),
            CornersCube::INDEX_SIZE,
        );
        println!("Corners table created in: {:?}", time.elapsed());

        table.save_to_file(path).unwrap();
    }

    // Verify data integrity
    let mut counts = vec![0; 12];
    for i in 0..CornersCube::INDEX_SIZE {
        let d = table.distance(i);
        counts[d as usize] += 1;
    }
    // According to https://oeis.org/A080629
    assert_eq!(counts, vec![1, 18, 243, 2874, 28000, 205416, 1168516, 5402628, 20776176, 45391616, 15139616, 64736]);

    table
}

pub fn subset_distance_table(twister: &Twister, path: &str) -> DistanceTable {
    let time = std::time::Instant::now();
    let result = DistanceTable::from_file(path);
    let table: DistanceTable;
    if result.is_ok() {
        println!("Subset table loaded in: {:?}", time.elapsed());
        table = result.unwrap();
    }
    else {
        table = DistanceTable::create(
            twister,
            TwistSet::h0(),
            SubsetCube::solved(),
            |s: SubsetCube| s.index(),
            |i: usize| SubsetCube::from_index(i),
            SubsetCube::INDEX_SIZE,
        );
        println!("Subset table created in: {:?}", time.elapsed());
        
        table.save_to_file(path).unwrap();
    }

    // Verify data integrity
    let mut counts = vec![0u64; 19];
    for i in 0..SubsetCube::INDEX_SIZE {
        let d = table.distance(i);
        counts[d as usize] += 1;
    }
    assert_eq!(counts, vec![
        1, 10, 67, 456, 3079, 19948, 123074, 736850, 4185118, 22630733,
        116767872, 552538680, 2176344160, 5627785188, 7172925794, 3608731814,
        224058996, 1575608, 1352
    ]);

    table
}

pub fn coset_direction_table(twister: &Twister, path: &str) -> DirectionsTable {
    let time = std::time::Instant::now();
    let result = DirectionsTable::from_file(path);
    let table: DirectionsTable;
    if result.is_ok() {
        println!("Coset table loaded in: {:?}", time.elapsed());
        table = result.unwrap();
    }
    else {
        table = DirectionsTable::create(
            twister,
            TwistSet::full(),
            CosetCube::solved(),
            |c: CosetCube| c.index(),
            |i: usize| CosetCube::from_index(i),
            CosetCube::INDEX_SIZE,
        );
        println!("Coset table created in: {:?}", time.elapsed());
        
        table.save_to_file(path).unwrap();
    }

    // Verify data integrity
    let mut counts = vec![0u64; 13];
    for i in 0..CosetCube::INDEX_SIZE {
        let d = table.distance(i);
        counts[d as usize] += 1;
    }
    assert_eq!(counts, vec![
        1, 4, 50, 592, 7156, 87236, 1043817, 12070278, 124946368, 821605960, 1199128738, 58202444, 476
    ]);

    table
}
