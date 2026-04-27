use super::Twistable;
use super::Twister;
use crate::cubies::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CosetIndex {
    pub c_ori: usize,       // 0..=2'186
    pub e_ori: usize,       // 0..=2'047
    pub e_slice_loc: usize, // 0..=494
}

impl CosetIndex {
    pub const INDEX_SIZE: usize = Corners::ORI_SIZE * Edges::ORI_SIZE * Edges::SLICE_LOC_SIZE; // 2'217'093'120

    pub fn solved() -> Self {
        let c = Corners::solved();
        let e = Edges::solved();
        Self {
            c_ori: c.ori_index(),
            e_ori: e.ori_index(),
            e_slice_loc: e.slice_loc_index(),
        }
    }

    pub fn in_subset(&self) -> bool {
        self.c_ori == 0 && self.e_ori == 0 && self.e_slice_loc == 494 // TODO: can we make this 0 by changing the definition of slice_loc_index?
    }

    pub fn index(&self) -> usize {
        self.c_ori * (Edges::ORI_SIZE * Edges::SLICE_LOC_SIZE)
            + self.e_ori * Edges::SLICE_LOC_SIZE
            + self.e_slice_loc
    }

    pub fn from_index(index: usize) -> Self {
        let mut index = index;
        let e_slice_loc = index % Edges::SLICE_LOC_SIZE;
        index /= Edges::SLICE_LOC_SIZE;
        let e_ori = index % Edges::ORI_SIZE;
        index /= Edges::ORI_SIZE;
        let c_ori = index;
        Self {
            c_ori,
            e_ori,
            e_slice_loc,
        }
    }

    pub fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        Self {
            c_ori: twister.twisted_c_ori(self.c_ori, twist),
            e_ori: twister.twisted_e_ori(self.e_ori, twist),
            e_slice_loc: twister.twisted_e_slice_loc(self.e_slice_loc, twist),
        }
    }

    pub fn twisted_by(&self, twister: &Twister, twists: &[Twist]) -> Self {
        twists
            .iter()
            .fold(*self, |index, &twist| index.twisted(twister, twist))
    }
}

impl fmt::Display for CosetIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CosetIndex {{ c_ori: {}, e_ori: {}, e_slice_loc: {} }}",
            self.c_ori, self.e_ori, self.e_slice_loc
        )
    }
}

impl Twistable for CosetIndex {
    fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        self.twisted(twister, twist)
    }

    fn twisted_by(&self, twister: &Twister, twists: &[Twist]) -> Self {
        self.twisted_by(twister, twists)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, RngExt, SeedableRng};

    // Tests 'index' and 'from_index'
    #[test]
    fn test_coset_index() {
        let mut rnd = StdRng::seed_from_u64(42);
        for _ in 0..100_000 {
            let rnd_index = rnd.random_range(0..CosetIndex::INDEX_SIZE);
            assert_eq!(CosetIndex::from_index(rnd_index).index(), rnd_index);
        }
    }
}
