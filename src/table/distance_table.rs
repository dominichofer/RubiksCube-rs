use crate::cubies::*;
use crate::index::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub struct DistanceTable {
    table: Vec<u8>,
}

impl DistanceTable {
    pub fn create<Obj>(
        twister: &Twister,
        twists: &[Twist],
        origin: Obj,
        index: impl Fn(Obj) -> usize + Sync,
        from_index: impl Fn(usize) -> Obj + Sync,
        index_size: usize,
    ) -> Self
    where
        Obj: Twistable + Send,
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
                        let next_index = index(obj.twisted(twister, *twist));
                        if table[next_index]
                            .compare_exchange(SENTINEL, d + 1, Ordering::Relaxed, Ordering::Relaxed)
                            .is_ok()
                        {
                            changed.store(true, Ordering::Relaxed);
                        }
                    }
                }
            });

            if !changed.load(Ordering::Relaxed) {
                break;
            }
        }
        Self {
            table: table
                .iter()
                .map(|x| x.load(Ordering::Relaxed))
                .collect::<Vec<u8>>(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        Ok(Self { table: std::fs::read(path)?,})
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, &self.table)
    }

    pub fn distance(&self, index: usize) -> u8 {
        self.table[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::twist_generator::RandomTwistGen;
    use crate::CornerIndex;

    #[test]
    fn test_distance_table() {
        let twister = Twister::new();
        let table = DistanceTable::create(
            &twister,
            &ALL_TWISTS,
            CornerIndex::solved(),
            |c: CornerIndex| c.index(),
            |i: usize| CornerIndex::from_index(i),
            CornerIndex::INDEX_SIZE,
        );

        let mut counts = vec![0; 12];
        for i in 0..CornerIndex::INDEX_SIZE {
            let d = table.distance(i);
            counts[d as usize] += 1;
        }
        // According to https://oeis.org/A080629
        assert_eq!(
            counts,
            vec![
                1, 18, 243, 2874, 28000, 205416, 1168516, 5402628, 20776176, 45391616, 15139616,
                64736
            ]
        );

        let mut rnd = RandomTwistGen::new(5989, &ALL_TWISTS);
        let mut cube = CornerIndex::solved();
        for _ in 0..100_000 {
            cube = cube.twisted(&twister, rnd.gen_twist());
            let d = table.distance(cube.index());

            // Check neighbours
            for twist in ALL_TWISTS {
                let neighbour_d = table.distance(cube.twisted(&twister, twist).index());
                assert!(
                    (neighbour_d as i32 - d as i32).abs() <= 1,
                    "Neighbour distance differs by more than 1 for cube at index {}",
                    cube.index()
                );
            }

            if d > 0 {
                // Check at least one neighbour has lower distance
                let mut found = false;
                for twist in ALL_TWISTS {
                    let neighbour_d = table.distance(cube.twisted(&twister, twist).index());
                    if neighbour_d < d {
                        found = true;
                        break;
                    }
                }
                assert!(
                    found,
                    "No neighbour with lower distance found for cube at index {}",
                    cube.index()
                );
            }
        }
    }
}
