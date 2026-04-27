use super::Twistable;
use super::Twister;
use crate::cubies::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubsetIndex {
    pub e_slice_prm: usize,     // 0..=23
    pub e_non_slice_prm: usize, // 0..=40'319
    pub c_prm: usize,           // 0..=40'319
}

impl SubsetIndex {
    pub const INDEX_SIZE: usize =
        Edges::SLICE_PRM_SIZE * Edges::NON_SLICE_PRM_SIZE * Corners::PRM_SIZE / 2; // 19'508'428'800

    pub fn solved() -> Self {
        let c = Corners::solved();
        let e = Edges::solved();
        Self {
            c_prm: c.prm_index(),
            e_slice_prm: e.slice_prm_index(),
            e_non_slice_prm: e.non_slice_prm_index(),
        }
    }

    pub fn index(&self) -> usize {
        (self.c_prm / 2) * Edges::SLICE_PRM_SIZE * Edges::NON_SLICE_PRM_SIZE
            + self.e_non_slice_prm * Edges::SLICE_PRM_SIZE
            + self.e_slice_prm
    }

    pub fn from_index(index: usize) -> Self {
        let mut index = index;
        let e_slice_prm = index % Edges::SLICE_PRM_SIZE;
        index /= Edges::SLICE_PRM_SIZE;
        let e_non_slice_prm = index % Edges::NON_SLICE_PRM_SIZE;
        index /= Edges::NON_SLICE_PRM_SIZE;
        let mut c_prm = index * 2;
        let e_even_prm = is_even_permutation(e_non_slice_prm as i64)
            ^ is_even_permutation(e_slice_prm as i64)
            ^ true; // in subset e_slice_loc is an even permutation
        if e_even_prm != is_even_permutation(c_prm as i64) {
            c_prm += 1;
        }
        Self {
            e_slice_prm,
            e_non_slice_prm,
            c_prm,
        }
    }

    pub fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        const SOLVED_SLICE_LOC_INDEX: usize = Edges::solved().slice_loc_index(); // TODO: can we make this 0 by changing the definition of slice_loc_index?
        SubsetIndex {
            c_prm: twister.twisted_c_prm(self.c_prm, twist),
            e_slice_prm: twister.twisted_e_slice_prm(
                self.e_slice_prm,
                SOLVED_SLICE_LOC_INDEX,
                twist,
            ),
            e_non_slice_prm: twister.twisted_e_non_slice_prm(
                self.e_non_slice_prm,
                SOLVED_SLICE_LOC_INDEX,
                twist,
            ),
        }
    }

    pub fn twisted_by(&self, twister: &Twister, twists: &[Twist]) -> Self {
        twists
            .iter()
            .fold(*self, |index, &twist| index.twisted(twister, twist))
    }
}

impl fmt::Display for SubsetIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SubsetIndex {{ e_slice_prm: {}, e_non_slice_prm: {}, c_prm: {} }}",
            self.e_slice_prm, self.e_non_slice_prm, self.c_prm
        )
    }
}

impl Twistable for SubsetIndex {
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
    fn test_subset_index() {
        let mut rnd = StdRng::seed_from_u64(42);
        for _ in 0..100_000 {
            let rnd_index = rnd.random_range(0..SubsetIndex::INDEX_SIZE);
            assert_eq!(SubsetIndex::from_index(rnd_index).index(), rnd_index);
        }
    }
}
