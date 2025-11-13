use crate::twist::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub struct DistanceTable {
    table: Vec<u8>,
    max_distance: u8,
}

impl DistanceTable {
    pub fn create<Obj, Index>(
        twists: Twists,
        origin: Obj,
        index: impl Fn(Obj) -> Index + Sync,
        from_index: impl Fn(usize) -> Obj + Sync,
        index_size: usize,
    ) -> Self
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
        let vec = table.iter().map(|x| x.load(Ordering::Relaxed)).collect::<Vec<u8>>();
        Self::from_vec(vec)
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        let max_distance = *vec.par_iter().max().unwrap();
        Self { table: vec, max_distance }
    }

    pub fn from_file(path: &str) -> Self {
        let data = std::fs::read(path).expect("Failed to read distance table file");
        Self::from_vec(data)
    }

    pub fn to_file(&self, path: &str) {
        std::fs::write(path, &self.table).expect("Failed to write distance table file");
    }

    pub fn len(&self) -> usize {
        self.table.len()
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
        
        let distance = self.distance(index(cube));
        for d in (1..=distance).rev() {
            for twist in twists.iter() {
                let next = cube.twisted(twist);
                let next_d = self.distance(index(next));
                if next_d < d {
                    solution.push(twist);
                    cube = next;
                    break;
                }
            }
        }
        assert_eq!(solution.len() as u8, distance);
        solution
    }
}

pub struct DirectionsAndDistance(u64);

impl DirectionsAndDistance {
    pub fn new(less: Twists, more: Twists, distance: u8) -> Self {
        Self(((less.bits() as u64) << 32) | ((more.bits() as u64) << 8) | (distance as u64))
    }

    pub fn from_u64(value: u64) -> Self {
        Self(value)
    }

    pub fn less_distance(&self) -> Twists {
        Twists::from_bits((self.0 >> 32) as u32)
    }

    pub fn more_distance(&self) -> Twists {
        Twists::from_bits(((self.0 >> 8) & 0xFF_FF_FF) as u32)
    }

    pub fn distance(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }
}

pub struct DirectionsTable {
    table: Vec<DirectionsAndDistance>,
    max_distance: u8,
}

impl DirectionsTable {
    pub fn create<Obj, Index>(
        twists: Twists,
        origin: Obj,
        index: impl Fn(Obj) -> Index + Sync,
        from_index: impl Fn(usize) -> Obj + Sync,
        index_size: usize,
    ) -> Self
    where
        Obj: Twistable + Copy,
        Index: Into<usize> + Copy,
    {
        let distance_table = DistanceTable::create(
            twists,
            origin,
            &index,
            &from_index,
            index_size,
        );
        let table: Vec<DirectionsAndDistance> = (0..index_size)
            .into_par_iter()
            .map(|i| {
                let d = distance_table.distance(i);
                let cube = from_index(i);
                let mut less = Twists::empty();
                let mut more = Twists::empty();

                for twist in twists.iter() {
                    let next = cube.twisted(twist);
                    let next_d = distance_table.distance(index(next).into());
                    if next_d < d {
                        less.set(twist);
                    } else if next_d > d {
                        more.set(twist);
                    }
                }

                DirectionsAndDistance::new(less, more, d)
            })
            .collect();
        let max_distance = distance_table.max_distance();
        Self { table, max_distance }
    }

    fn from_vec(vec: Vec<DirectionsAndDistance>) -> Self {
        let max_distance = vec.iter().map(|d| d.distance()).max().unwrap();
        Self { table: vec, max_distance }
    }

    pub fn from_file(path: &str) -> Self {
        let data = std::fs::read(path).expect("Failed to read directions table file");
        let mut table: Vec<DirectionsAndDistance> = Vec::with_capacity(data.len() / 8);
        for chunk in data.chunks_exact(8) {
            let value = u64::from_le_bytes(chunk.try_into().unwrap());
            table.push(DirectionsAndDistance::from_u64(value));
        }
        Self::from_vec(table)
    }

    pub fn to_file(&self, path: &str) {
        let mut data: Vec<u8> = Vec::with_capacity(self.table.len() * 8);
        for value in &self.table {
            data.extend_from_slice(&value.0.to_le_bytes());
        }
        std::fs::write(path, &data).expect("Failed to write directions table file");
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn distance(&self, index: usize) -> u8 {
        self.table[index].distance()
    }

    pub fn less_distance(&self, index: usize) -> Twists {
        self.table[index].less_distance()
    }

    pub fn more_distance(&self, index: usize) -> Twists {
        self.table[index].more_distance()
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
    fn test_distance_table() {
        let table = DistanceTable::create(
            Twists::all(),
            Corners::solved(),
            |c: Corners| c.index() as usize,
            |i: usize| Corners::from_combined_index(i as u32),
            Corners::INDEX_SIZE as usize,
        );
        
        let mut counts = vec![0; 12];
        for i in 0..table.len() {
            let d = table.distance(i) as usize;
            counts[d] += 1;
        }
        // According to https://oeis.org/A080629
        assert_eq!(counts, vec![
            1, 18, 243, 2_874, 28_000, 205_416,
            1_168_516, 5_402_628, 20_776_176, 45_391_616,
            15_139_616, 64_736
        ]);
        assert_eq!(table.max_distance(), 11);

        
        let mut rnd = RandomTwistGen::new(5989, Twists::all());
        for _ in 0..100_000 {
            let mut cube = Corners::solved();
            let twists = rnd.gen_twists(100);
            for &twist in &twists {
                cube = cube.twisted(twist);
            }
            let solution = table.solution(cube, Twists::all(), |c: Corners| c.index() as usize);
            let mut test_cube = cube;
            for &twist in &solution {
                test_cube = test_cube.twisted(twist);
            }
            assert_eq!(test_cube, Corners::solved());
        }
    }

    #[test]
    fn test_directions_table() {
        let table = DirectionsTable::create(
            Twists::all(),
            Corners::solved(),
            |c: Corners| c.index() as usize,
            |i: usize| Corners::from_combined_index(i as u32),
            Corners::INDEX_SIZE as usize,
        );

        for _ in 0..100_000 {
            let i = rand::random::<usize>() % Corners::INDEX_SIZE as usize;
            let d = table.distance(i);
            let less = table.less_distance(i);
            let more = table.more_distance(i);

            let cube = Corners::from_combined_index(i as u32);
            for twist in Twists::all().iter() {
                let next = cube.twisted(twist);
                let next_d = table.distance(next.index() as usize);
                if next_d < d {
                    assert!(less.contains(twist), "Less missing twist {:?} at index {}", twist, i);
                } else if next_d > d {
                    assert!(more.contains(twist), "More missing twist {:?} at index {}", twist, i);
                }
            }
        }
    }
}
