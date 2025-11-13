use crate::corners::*;
use crate::edges::*;
use crate::is_even_permutation;
use crate::twist::*;
use crate::twister::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubsetCube {
    e_slice_prm: u8, // 0..=23
    e_non_slice_prm: u16, // 0..=40'319
    c_prm: u16, // 0..=40'319
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CosetCube {
    c_ori: u16, // 0..=2'186
    e_ori: u16, // 0..=2'047
    e_slice_loc: u16, // 0..=494
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cube {
    pub c_prm: u16, // 0..=40'319
    pub c_ori: u16, // 0..=2'186
    pub e_non_slice_prm: u16, // 0..=40'319
    pub e_slice_prm: u8, // 0..=23
    pub e_slice_loc: u16, // 0..=494
    pub e_ori: u16, // 0..=2'047
}

impl SubsetCube {
    pub const INDEX_SIZE: u64 = (Edges::SLICE_PRM_SIZE as u64) * (Edges::NON_SLICE_PRM_SIZE as u64) * (Corners::PRM_SIZE as u64) / 2; // 19'508'428'800

    pub fn solved() -> Self {
        let c = Corners::solved();
        let e = Edges::solved();
        Self {
            c_prm: c.prm_index() as u16,
            e_slice_prm: e.slice_prm_index() as u8,
            e_non_slice_prm: e.non_slice_prm_index() as u16,
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::solved()
    }

    pub fn index(&self) -> u64 {
        const SLICE_PRM_SIZE: u64 = Edges::SLICE_PRM_SIZE as u64;
        const NON_SLICE_PRM_SIZE: u64 = Edges::NON_SLICE_PRM_SIZE as u64;

        (self.c_prm as u64 / 2) * SLICE_PRM_SIZE * NON_SLICE_PRM_SIZE
        + (self.e_non_slice_prm as u64) * SLICE_PRM_SIZE
        + (self.e_slice_prm as u64)
    }

    pub fn from_index(index: u64) -> Self {
        const SLICE_PRM_SIZE: u64 = Edges::SLICE_PRM_SIZE as u64;
        const NON_SLICE_PRM_SIZE: u64 = Edges::NON_SLICE_PRM_SIZE as u64;

        let mut index = index;
        let e_slice_prm = (index % SLICE_PRM_SIZE) as u8;
        index /= SLICE_PRM_SIZE;
        let e_non_slice_prm = (index % NON_SLICE_PRM_SIZE) as u16;
        index /= NON_SLICE_PRM_SIZE;
        let mut c_prm = (index * 2) as u16;

        let e_even_prm = is_even_permutation(e_non_slice_prm as i64)
            ^ is_even_permutation(e_slice_prm as i64)
            ^ true; // in subset e_slice_loc is an even permutation
        if e_even_prm != is_even_permutation(c_prm as i64) {
            c_prm += 1;
        }

        SubsetCube { e_slice_prm, e_non_slice_prm, c_prm }
    }
}

impl CosetCube {
    pub const INDEX_SIZE: u32 = (Corners::ORI_SIZE as u32) * (Edges::ORI_SIZE as u32) * (Edges::SLICE_LOC_SIZE as u32); // 2'217'093'120

    pub fn solved() -> Self {
        let c = Corners::solved();
        let e = Edges::solved();
        Self {
            c_ori: c.ori_index() as u16,
            e_ori: e.ori_index() as u16,
            e_slice_loc: e.slice_loc_index() as u16,
        }
    }

    pub fn in_subset(&self) -> bool {
        *self == Self::solved()
    }

    pub fn index(&self) -> u32 {
        const E_ORI_SIZE: u32 = Edges::ORI_SIZE as u32;
        const SLICE_LOC_SIZE: u32 = Edges::SLICE_LOC_SIZE as u32;

        (self.c_ori as u32) * (E_ORI_SIZE * SLICE_LOC_SIZE)
        + (self.e_ori as u32) * SLICE_LOC_SIZE
        + (self.e_slice_loc as u32)
    }

    pub fn from_index(index: u32) -> Self {
        const E_ORI_SIZE: u32 = Edges::ORI_SIZE as u32;
        const SLICE_LOC_SIZE: u32 = Edges::SLICE_LOC_SIZE as u32;

        let mut index = index;
        let e_slice_loc = (index % SLICE_LOC_SIZE) as u16;
        index /= SLICE_LOC_SIZE;
        let e_ori = (index % E_ORI_SIZE) as u16;
        index /= E_ORI_SIZE;
        let c_ori = index as u16;

        CosetCube { c_ori, e_ori, e_slice_loc }
    }
}

impl Cube {
    pub fn solved() -> Self {
        let c = Corners::solved();
        let e = Edges::solved();
        Self {
            c_ori: c.ori_index() as u16,
            c_prm: c.prm_index() as u16,
            e_non_slice_prm: e.non_slice_prm_index() as u16,
            e_slice_prm: e.slice_prm_index() as u8,
            e_slice_loc: e.slice_loc_index() as u16,
            e_ori: e.ori_index() as u16,
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::solved()
    }

    pub fn in_subset(&self) -> bool {
        self.c_ori == 0
            && self.e_ori == 0
            && self.e_slice_loc == 494
    }

    pub fn to_subset(&self) -> SubsetCube {
        SubsetCube {
            c_prm: self.c_prm,
            e_slice_prm: self.e_slice_prm,
            e_non_slice_prm: self.e_non_slice_prm,
        }
    }

    pub fn coset_index(&self) -> u32 {
        let coset_cube = CosetCube {
            c_ori: self.c_ori,
            e_ori: self.e_ori,
            e_slice_loc: self.e_slice_loc,
        };
        coset_cube.index()
    }

    pub fn corners_index(&self) -> u32 {
        Corners::combine_indices(self.c_prm, self.c_ori)
    }
}

impl Twistable for SubsetCube {
    fn twisted(&self, twist: Twist) -> Self {
        let solved_e_slice_loc = Edges::solved().slice_loc_index();
        Self {
            c_prm: TWISTER.twisted_c_prm(self.c_prm, twist),
            e_slice_prm: TWISTER.twisted_e_slice_prm(self.e_slice_prm, solved_e_slice_loc, twist),
            e_non_slice_prm: TWISTER.twisted_e_non_slice_prm(self.e_non_slice_prm, solved_e_slice_loc, twist),
        }
    }
}

impl Twistable for CosetCube {
    fn twisted(&self, twist: Twist) -> Self {
        Self {
            c_ori: TWISTER.twisted_c_ori(self.c_ori, twist),
            e_ori: TWISTER.twisted_e_ori(self.e_ori, twist),
            e_slice_loc: TWISTER.twisted_e_slice_loc(self.e_slice_loc, twist),
        }
    }
}

impl Twistable for Cube {
    fn twisted(&self, twist: Twist) -> Self {
        Self {
            c_prm: TWISTER.twisted_c_prm(self.c_prm, twist),
            c_ori: TWISTER.twisted_c_ori(self.c_ori, twist),
            e_non_slice_prm: TWISTER.twisted_e_non_slice_prm(self.e_non_slice_prm, self.e_slice_loc, twist),
            e_slice_prm: TWISTER.twisted_e_slice_prm(self.e_slice_prm, self.e_slice_loc, twist),
            e_slice_loc: TWISTER.twisted_e_slice_loc(self.e_slice_loc, twist),
            e_ori: TWISTER.twisted_e_ori(self.e_ori, twist),
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

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cube {{ c_prm: {}, c_ori: {}, e_non_slice_prm: {}, e_slice_prm: {}, e_slice_loc: {}, e_ori: {} }}",
            self.c_prm, self.c_ori, self.e_non_slice_prm, self.e_slice_prm, self.e_slice_loc, self.e_ori
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_twisted() {
        let mut rnd = RandomTwistGen::new(42, Twists::h0());
        let mut cube = SubsetCube::solved();
        let mut c = Corners::solved();
        let mut e = Edges::solved();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            cube = cube.twisted(twist);
            c = c.twisted(twist);
            e = e.twisted(twist);
            assert_eq!(cube.c_prm, c.prm_index());
            assert_eq!(cube.e_slice_prm, e.slice_prm_index());
            assert_eq!(cube.e_non_slice_prm, e.non_slice_prm_index());
        }
    }

    #[test]
    fn test_coset_twisted() {
        let mut rnd = RandomTwistGen::new(42, Twists::all());
        let mut cube = CosetCube::solved();
        let mut c = Corners::solved();
        let mut e = Edges::solved();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            cube = cube.twisted(twist);
            c = c.twisted(twist);
            e = e.twisted(twist);
            assert_eq!(cube.c_ori, c.ori_index());
            assert_eq!(cube.e_ori, e.ori_index());
            assert_eq!(cube.e_slice_loc, e.slice_loc_index());
        }
    }

    #[test]
    fn test_subset_index() {
        let mut rnd = RandomTwistGen::new(42, Twists::h0());
        let mut cube = SubsetCube::solved();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            cube = cube.twisted(twist);
            let index = cube.index();
            assert!(index < SubsetCube::INDEX_SIZE);
            assert_eq!(cube, SubsetCube::from_index(index));
        }
    }

    #[test]
    fn test_coset_index() {
        let mut rnd = RandomTwistGen::new(42, Twists::all());
        let mut cube = CosetCube::solved();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            cube = cube.twisted(twist);
            let index = cube.index();
            assert!(index < CosetCube::INDEX_SIZE);
            assert_eq!(cube, CosetCube::from_index(index));
        }
    }
}
