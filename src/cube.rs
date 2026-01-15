use crate::corners::*;
use crate::edges::*;
use crate::twist::Twist;
use crate::twister::*;
use crate::is_even_permutation;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CornersCube {
    pub prm: usize, // 0..=40'319
    pub ori: usize, // 0..=2'186
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubsetCube {
    pub e_slice_prm: usize, // 0..=23
    pub e_non_slice_prm: usize, // 0..=40'319
    pub c_prm: usize, // 0..=40'319
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CosetCube {
    pub c_ori: usize, // 0..=2'186
    pub e_ori: usize, // 0..=2'047
    pub e_slice_loc: usize, // 0..=494
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cube {
    pub subset: SubsetCube,
    pub coset: CosetCube,
}

impl CornersCube {
    pub const INDEX_SIZE: usize = Corners::PRM_SIZE * Corners::ORI_SIZE; // 88'179'840

    pub fn solved() -> Self {
        let c = Corners::solved();
        Self {
            prm: c.prm_index(),
            ori: c.ori_index(),
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::solved()
    }

    pub fn index(&self) -> usize {
        self.prm * Corners::ORI_SIZE + self.ori
    }

    pub fn from_index(index: usize) -> Self {
        let prm = index / Corners::ORI_SIZE;
        let ori = index % Corners::ORI_SIZE;
        CornersCube { prm, ori }
    }
}

impl SubsetCube {
    pub const INDEX_SIZE: usize = Edges::SLICE_PRM_SIZE * Edges::NON_SLICE_PRM_SIZE * Corners::PRM_SIZE / 2; // 19'508'428'800

    pub fn solved() -> Self {
        let c = Corners::solved();
        let e = Edges::solved();
        Self {
            c_prm: c.prm_index(),
            e_slice_prm: e.slice_prm_index(),
            e_non_slice_prm: e.non_slice_prm_index(),
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::solved()
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
        Self { e_slice_prm, e_non_slice_prm, c_prm }
    }
}

impl CosetCube {
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
        self.c_ori == 0
            && self.e_ori == 0
            && self.e_slice_loc == 494
    }

    pub fn index(&self) -> usize {
        self.c_ori * (Edges::ORI_SIZE * Edges::SLICE_LOC_SIZE)
        + self.e_ori * Edges::SLICE_LOC_SIZE
        + self.e_slice_loc
    }

    pub fn from_index(index: usize) -> Self {
        let mut index = index;
        let e_slice_loc = index % Edges::SLICE_LOC_SIZE; index /= Edges::SLICE_LOC_SIZE;
        let e_ori = index % Edges::ORI_SIZE; index /= Edges::ORI_SIZE;
        let c_ori = index;
        Self { c_ori, e_ori, e_slice_loc }
    }
}

impl Cube {
    pub fn solved() -> Self {
        Self {
            subset: SubsetCube::solved(),
            coset: CosetCube::solved(),
        }
    }

    pub fn is_solved(&self) -> bool {
        self.subset.is_solved() && self.coset.in_subset()
    }
}

impl Twistable for CornersCube {
    fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        Self {
            prm: twister.twisted_c_prm(self.prm, twist),
            ori: twister.twisted_c_ori(self.ori, twist),
        }
    }
}

impl Twistable for SubsetCube {
    fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        Self {
            e_slice_prm: twister.twisted_e_slice_prm(self.e_slice_prm, Edges::solved().slice_loc_index(), twist),
            e_non_slice_prm: twister.twisted_e_non_slice_prm(self.e_non_slice_prm, Edges::solved().slice_loc_index(), twist),
            c_prm: twister.twisted_c_prm(self.c_prm, twist),
        }
    }
}

impl Twistable for CosetCube {
    fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        Self {
            c_ori: twister.twisted_c_ori(self.c_ori, twist),
            e_ori: twister.twisted_e_ori(self.e_ori, twist),
            e_slice_loc: twister.twisted_e_slice_loc(self.e_slice_loc, twist),
        }
    }
}

impl Twistable for Cube {
    fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        Self {
            subset: SubsetCube {
                e_slice_prm: twister.twisted_e_slice_prm(self.subset.e_slice_prm, self.coset.e_slice_loc, twist),
                e_non_slice_prm: twister.twisted_e_non_slice_prm(self.subset.e_non_slice_prm, self.coset.e_slice_loc, twist),
                c_prm: twister.twisted_c_prm(self.subset.c_prm, twist),
            },
            coset: self.coset.twisted(twister, twist),
        }
    }
}

impl fmt::Display for SubsetCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SubsetCube {{ e_slice_prm: {}, e_non_slice_prm: {}, c_prm: {} }}",
            self.e_slice_prm, self.e_non_slice_prm, self.c_prm
        )
    }
}

impl fmt::Display for CosetCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CosetCube {{ c_ori: {}, e_ori: {}, e_slice_loc: {} }}",
            self.c_ori, self.e_ori, self.e_slice_loc
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_corners_index() {
        let mut rnd = rand::rngs::StdRng::seed_from_u64(42);
        for _ in 0..100_000 {
            let index = rnd.random_range(0..CornersCube::INDEX_SIZE);
            assert_eq!(CornersCube::from_index(index).index(), index);
        }
    }

    #[test]
    fn test_subset_index() {
        let mut rnd = rand::rngs::StdRng::seed_from_u64(42);
        for _ in 0..100_000 {
            let index = rnd.random_range(0..SubsetCube::INDEX_SIZE);
            assert_eq!(SubsetCube::from_index(index).index(), index);
        }
    }

    #[test]
    fn test_coset_index() {
        let mut rnd = rand::rngs::StdRng::seed_from_u64(42);
        for _ in 0..100_000 {
            let index = rnd.random_range(0..CosetCube::INDEX_SIZE);
            assert_eq!(CosetCube::from_index(index).index(), index);
        }
    }
}
