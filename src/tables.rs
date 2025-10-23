use crate::twist::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub fn create_distance_table<Obj, Index>(
    twists: Twists,
    origin: Obj,
    index: impl Fn(Obj) -> Index + Sync,
    from_index: impl Fn(usize) -> Obj + Sync,
    index_size: usize,
) -> Vec<AtomicU8>
where
    Obj: Twistable + Copy,
    Index: Into<usize> + Copy,
{
    const SENTINEL: u8 = u8::MAX;
    let table: Vec<AtomicU8> = (0..index_size)
        .into_par_iter()
        .map(|_| AtomicU8::new(SENTINEL))
        .collect();

    table[index(origin).into()].store(0, Ordering::Relaxed);

    for d in 0..SENTINEL - 1 {
        let changed = AtomicBool::new(false);

        (0..table.len()).into_par_iter().for_each(|i| {
            if table[i].load(Ordering::Acquire) == d {
                let cube = from_index(i);
                for twist in twists.iter() {
                    let next = index(cube.twisted(twist)).into();
                    if table[next].compare_exchange(SENTINEL, d + 1, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                        changed.store(true, Ordering::Relaxed);
                    }
                }
            }
        });

        if !changed.load(Ordering::Acquire) {
            break;
        }
    }
    table
}

pub fn store_distance_table(path: &str, table: &[AtomicU8]) {
    let file = std::fs::File::create(path).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    std::io::Write::write_all(
        &mut writer,
        &table.iter().map(|x| x.load(Ordering::Relaxed)).collect::<Vec<u8>>(),
    ).unwrap();
}

pub struct DistanceTable {
    table: Vec<u8>,
    max_distance: u8,
}

impl DistanceTable {
    pub fn from_file(path: &str) -> Self {
        let table = std::fs::read(path).unwrap();
        let max_distance = *table.iter().max().unwrap();
        Self { table, max_distance }
    }

    pub fn distance(&self, index: usize) -> u8 {
        self.table[index]
    }

    pub fn max_distance(&self) -> u8 {
        self.max_distance
    }

    pub fn solution<Obj>(&self, cube: Obj, twists: Twists, index: impl Fn(Obj) -> usize) -> Vec<Twist>
    where
        Obj: Twistable + Copy,
    {
        let mut cube = cube;
        let mut solution = Vec::new();
        
        let mut d = self.distance(index(cube));
        while d > 0 {
            for twist in twists.iter() {
                let next = cube.twisted(twist);
                let next_d = self.distance(index(next));
                if next_d < d {
                    solution.push(twist);
                    cube = next;
                    d = next_d;
                    break;
                }
            }
        }

        solution
    }
}

pub fn create_directions_table<Obj, Index>(
    twists: Twists,
    origin: Obj,
    index: impl Fn(Obj) -> Index + Sync,
    from_index: impl Fn(usize) -> Obj + Sync,
    index_size: usize,
) -> Vec<u64>
where
    Obj: Twistable + Copy,
    Index: Into<usize> + Copy,
{
    let distance_table = create_distance_table(
        twists,
        origin,
        &index,
        &from_index,
        index_size,
    );
    let table: Vec<u64> = (0..index_size)
        .into_par_iter()
        .map(|i| {
            let d = distance_table[i].load(Ordering::Relaxed);
            let cube = from_index(i);
            let mut less = Twists::empty();
            let mut more = Twists::empty();

            for twist in twists.iter() {
                let next = cube.twisted(twist);
                let next_d = distance_table[index(next).into()].load(Ordering::Relaxed);
                if next_d < d {
                    less.set(twist);
                } else if next_d > d {
                    more.set(twist);
                }
            }

            ((less.bits() as u64) << 32) | ((more.bits() as u64) << 8) | (d as u64)
        })
        .collect();

    table
}

pub fn store_directions_table(path: &str, table: &[u64]) {
    let file = std::fs::File::create(path).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    std::io::Write::write_all(
        &mut writer,
        table.iter().flat_map(|&x| x.to_le_bytes()).collect::<Vec<u8>>().as_slice(),
    ).unwrap();
}

pub struct DirectionsTable {
    table: Vec<u64>,
    max_distance: u8,
}

impl DirectionsTable {
    pub fn from_file(pth: &str) -> Self {
        let data = std::fs::read(pth).unwrap();
        let table: Vec<u64> = data
            .chunks_exact(9)
            .map(|chunk| {
                let less = Twists::from_bits(u32::from_le_bytes(chunk[0..4].try_into().unwrap()));
                let more = Twists::from_bits(u32::from_le_bytes(chunk[4..8].try_into().unwrap()));
                let distance = chunk[8];
                ((less.bits() as u64) << 32) | ((more.bits() as u64) << 8) | (distance as u64)
            })
            .collect();
        let max_distance = table.iter().map(|d| d & 0xFF).max().unwrap() as u8;
        Self { table, max_distance }
    }

    pub fn distance(&self, index: usize) -> u8 {
        self.table[index] as u8
    }

    pub fn less_distance(&self, index: usize) -> Twists {
        Twists::from_bits((self.table[index] >> 32) as u32)
    }

    pub fn more_distance(&self, index: usize) -> Twists {
        Twists::from_bits((self.table[index] >> 8) as u32)
    }

    pub fn max_distance(&self) -> u8 {
        self.max_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corners::Corners;

    #[test]
    fn test_corners() {
        let table = create_distance_table(
            Twists::all(),
            Corners::solved(),
            |c: Corners| c.index() as usize,
            |i: usize| Corners::from_combined_index(i as u32),
            Corners::INDEX_SIZE as usize,
        );

        // According to https://oeis.org/A080629
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 0).count(), 1);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 1).count(), 18);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 2).count(), 243);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 3).count(), 2_874);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 4).count(), 28_000);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 5).count(), 205_416);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 6).count(), 1_168_516);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 7).count(), 5_402_628);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 8).count(), 20_776_176);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 9).count(), 45_391_616);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 10).count(), 15_139_616);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 11).count(), 64_736);
        assert_eq!(table.iter().filter(|&x| x.load(Ordering::Relaxed) == 12).count(), 0);
    }

    #[test]
    fn test_directions_table() {
        let table = create_directions_table(
            Twists::all(),
            Corners::solved(),
            |c: Corners| c.index() as usize,
            |i: usize| Corners::from_combined_index(i as u32),
            Corners::INDEX_SIZE as usize,
        );

        for _ in 0..100_000 {
            let i = rand::random::<usize>() % Corners::INDEX_SIZE as usize;
            let d = (table[i] & 0xFF) as u64;
            let less = Twists::from_bits((table[i] >> 32) as u32);
            let more = Twists::from_bits((table[i] >> 8) as u32);

            let cube = Corners::from_combined_index(i as u32);
            for twist in Twists::all().iter() {
                let next = cube.twisted(twist);
                let next_d = table[next.index() as usize] & 0xFF;
                if next_d < d {
                    assert!(less.contains(twist), "Less missing twist {:?} at index {}", twist, i);
                } else if next_d > d {
                    assert!(more.contains(twist), "More missing twist {:?} at index {}", twist, i);
                }
            }
        }
    }
}
