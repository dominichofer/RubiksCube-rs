use crate::cubies::*;
use crate::index::*;
use crate::table::DistanceTable;
use rayon::prelude::*;

pub struct DirectionsAndDistance(u64);

impl DirectionsAndDistance {
    pub fn new(less: TwistSet, more: TwistSet, distance: u8) -> Self {
        Self(((less.bits() as u64) << 32) | ((more.bits() as u64) << 8) | (distance as u64))
    }

    pub fn from_u64(value: u64) -> Self {
        Self(value)
    }

    pub fn less_distance(&self) -> TwistSet {
        TwistSet::from((self.0 >> 32) as u32)
    }

    pub fn more_distance(&self) -> TwistSet {
        TwistSet::from(((self.0 >> 8) & 0xFF_FF_FF) as u32)
    }

    pub fn distance(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }
}

pub struct DirectionsTable {
    table: Vec<DirectionsAndDistance>,
}

impl DirectionsTable {
    pub fn create<Obj: Twistable + Send>(
        twister: &Twister,
        twists: &[Twist],
        origin: Obj,
        index: impl Fn(Obj) -> usize + Sync,
        from_index: impl Fn(usize) -> Obj + Sync,
        index_size: usize,
    ) -> Self {
        let distance_table =
            DistanceTable::create(twister, twists, origin, &index, &from_index, index_size);
        let table: Vec<DirectionsAndDistance> = (0..index_size)
            .into_par_iter()
            .map(|i| {
                let d = distance_table.distance(i);
                let obj = from_index(i);
                let mut less = TwistSet::EMPTY;
                let mut more = TwistSet::EMPTY;

                for twist in twists {
                    let next = obj.twisted(twister, *twist);
                    let next_d = distance_table.distance(index(next));
                    if next_d < d {
                        less.set_twist(*twist);
                    } else if next_d > d {
                        more.set_twist(*twist);
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

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let mut data = Vec::with_capacity(self.table.len() * 8);
        for entry in &self.table {
            data.extend_from_slice(&entry.0.to_le_bytes());
        }
        std::fs::write(path, data)
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
    use crate::CornerIndex;
    use rand::{rngs::StdRng, RngExt, SeedableRng};

    #[test]
    fn test_directions_table() {
        let mut rnd = StdRng::seed_from_u64(42);
        let twister = Twister::new();
        let table = DirectionsTable::create(
            &twister,
            &ALL_TWISTS,
            CornerIndex::solved(),
            |c: CornerIndex| c.index(),
            |i: usize| CornerIndex::from_index(i),
            CornerIndex::INDEX_SIZE,
        );

        for _ in 0..100_000 {
            let i = rnd.random_range(0..CornerIndex::INDEX_SIZE);
            let d = table.distance(i);
            let less = table.less_distance(i);
            let more = table.more_distance(i);

            let cube = CornerIndex::from_index(i);
            for twist in ALL_TWISTS {
                let next = cube.twisted(&twister, twist);
                let next_d = table.distance(next.index());
                if next_d < d {
                    assert!(
                        less.contains(twist),
                        "Less missing twist {:?} at index {}",
                        twist,
                        i
                    );
                } else if next_d > d {
                    assert!(
                        more.contains(twist),
                        "More missing twist {:?} at index {}",
                        twist,
                        i
                    );
                }
            }
        }
    }
}
