use crate::twist::*;
use crate::twister::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub struct DistanceTable {
    table: Vec<u8>,
}

impl DistanceTable {
    pub fn create<Obj>(
        twister: &Twister,
        twists: TwistSet,
        origin: Obj,
        index: impl Fn(Obj) -> usize + Sync,
        from_index: impl Fn(usize) -> Obj + Sync,
        index_size: usize,
    ) -> Self
    where
        Obj: Twistable,
    {
        const SENTINEL: u8 = u8::MAX;
        let table: Vec<AtomicU8> = (0..index_size)
            .into_par_iter()
            .map(|_| AtomicU8::new(SENTINEL))
            .collect();

        table[index(origin)].store(0, Ordering::Release);

        for d in 0..SENTINEL - 1 {
            let changed = AtomicBool::new(false);

            (0..table.len()).into_par_iter().for_each(|i| {
                if table[i].load(Ordering::Relaxed) == d {
                    let obj = from_index(i);
                    for twist in twists.iter() {
                        let next_index = index(obj.twisted(twister, twist));
                        if table[next_index].compare_exchange(SENTINEL, d + 1, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
                            changed.store(true, Ordering::Relaxed);
                        }
                    }
                }
            });

            if !changed.load(Ordering::Relaxed) {
                break;
            }
        }
        Self { table: table.iter().map(|x| x.load(Ordering::Relaxed)).collect::<Vec<u8>>() }
    }

    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let table = std::fs::read(path)?;
        Ok(Self { table })
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, &self.table)?;
        Ok(())
    }

    pub fn distance(&self, index: usize) -> u8 {
        self.table[index]
    }
}

pub struct DirectionsAndDistance(u64);

impl DirectionsAndDistance {
    pub fn new(less: TwistSet, more: TwistSet, distance: u8) -> Self {
        Self(((less.bits() as u64) << 32) | ((more.bits() as u64) << 8) | (distance as u64))
    }

    pub fn from_u64(value: u64) -> Self {
        Self(value)
    }

    pub fn less_distance(&self) -> TwistSet {
        TwistSet::from_bits((self.0 >> 32) as u32)
    }

    pub fn more_distance(&self) -> TwistSet {
        TwistSet::from_bits(((self.0 >> 8) & 0xFF_FF_FF) as u32)
    }

    pub fn distance(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }
}

pub struct DirectionsTable {
    table: Vec<DirectionsAndDistance>,
}

impl DirectionsTable {
    pub fn create<Obj>(
        twister: &Twister,
        twists: TwistSet,
        origin: Obj,
        index: impl Fn(Obj) -> usize + Sync,
        from_index: impl Fn(usize) -> Obj + Sync,
        index_size: usize,
    ) -> Self
    where
        Obj: Twistable,
    {
        let distance_table = DistanceTable::create(
            twister,
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
                let obj = from_index(i);
                let mut less = TwistSet::empty();
                let mut more = TwistSet::empty();

                for twist in twists.iter() {
                    let next = obj.twisted(twister, twist);
                    let next_d = distance_table.distance(index(next));
                    if next_d < d {
                        less.set(twist);
                    } else if next_d > d {
                        more.set(twist);
                    }
                }

                DirectionsAndDistance::new(less, more, d)
            })
            .collect();
        Self { table }
    }

    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let data = std::fs::read(path)?;
        let table: Vec<DirectionsAndDistance> = data
            .chunks_exact(8)
            .map(|chunk| {
                let value = u64::from_le_bytes(chunk.try_into().unwrap());
                DirectionsAndDistance::from_u64(value)
            })
            .collect();
        Ok(Self { table })
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let data: Vec<u8> = self.table
            .par_iter()
            .flat_map(|value| value.0.to_le_bytes())
            .collect();
        std::fs::write(path, &data)?;
        Ok(())
    }

    pub fn distance(&self, index: usize) -> u8 {
        self.table[index].distance()
    }

    pub fn less_distance(&self, index: usize) -> TwistSet {
        self.table[index].less_distance()
    }

    pub fn more_distance(&self, index: usize) -> TwistSet {
        self.table[index].more_distance()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::CornersCube;
    use rand::Rng;

    #[test]
    fn test_distance_table() {
        let twister = Twister::new();
        let table = DistanceTable::create(
            &twister,
            TwistSet::full(),
            CornersCube::solved(),
            |c: CornersCube| c.index(),
            |i: usize| CornersCube::from_index(i),
            CornersCube::INDEX_SIZE,
        );
        
        let mut counts = vec![0; 12];
        for i in 0..CornersCube::INDEX_SIZE {
            let d = table.distance(i);
            counts[d as usize] += 1;
        }
        // According to https://oeis.org/A080629
        assert_eq!(counts, vec![1, 18, 243, 2874, 28000, 205416, 1168516, 5402628, 20776176, 45391616, 15139616, 64736]);

        let mut rnd = RandomTwistGen::new(5989, TwistSet::full());
        let mut cube = CornersCube::solved();
        for _ in 0..100_000 {
            cube = cube.twisted(&twister, rnd.gen_twist());
            let d = table.distance(cube.index());

            // Check neighbours
            for twist in TwistSet::full().iter() {
                let neighbour_d = table.distance(cube.twisted(&twister, twist).index());
                assert!((neighbour_d as i32 - d as i32).abs() <= 1, "Neighbour distance differs by more than 1 for cube at index {}", cube.index());
            }

            if d > 0 {
                // Check at least one neighbour has lower distance
                let mut found = false;
                for twist in TwistSet::full().iter() {
                    let neighbour_d = table.distance(cube.twisted(&twister, twist).index());
                    if neighbour_d < d {
                        found = true;
                        break;
                    }
                }
                assert!(found, "No neighbour with lower distance found for cube at index {}", cube.index());
            }
            
        }
    }

    #[test]
    fn test_directions_table() {
        let twister = Twister::new();
        let table = DirectionsTable::create(
            &twister,
            TwistSet::full(),
            CornersCube::solved(),
            |c: CornersCube| c.index(),
            |i: usize| CornersCube::from_index(i),
            CornersCube::INDEX_SIZE,
        );

        for _ in 0..100_000 {
            let i = rand::rng().random_range(0..CornersCube::INDEX_SIZE);
            let d = table.distance(i);
            let less = table.less_distance(i);
            let more = table.more_distance(i);

            let cube = CornersCube::from_index(i);
            for twist in TwistSet::full().iter() {
                let next = cube.twisted(&twister, twist);
                let next_d = table.distance(next.index());
                if next_d < d {
                    assert!(less.contains(twist), "Less missing twist {:?} at index {}", twist, i);
                } else if next_d > d {
                    assert!(more.contains(twist), "More missing twist {:?} at index {}", twist, i);
                }
            }
        }
    }
}
