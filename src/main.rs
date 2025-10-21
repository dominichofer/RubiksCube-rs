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

fn main() {
    let corners_table = corners_distance_table("D:\\corners_distance_table.bin");
    let subset_table = subset_distance_table("D:\\subset_distance_table.bin");
    let coset_table = coset_distance_table("D:\\coset_distance_table.bin");
}
