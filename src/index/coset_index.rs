use super::Twistable;
use super::TWISTER;
use crate::math::binomial;
use crate::cubies::*;
// use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CosetIndex {
    pub c_ori: usize, // 3^7 = 2'187
    pub e_ori: usize, // 2^11 = 2'048
    pub z_loc: usize, // (12 choose 4) = 495
}

impl CosetIndex {
    pub const INDEX_SIZE: usize = Corners::ORI_SIZE * Edges::ORI_SIZE * binomial(12, 4); // 2'217'093'120

    pub fn solved() -> Self {
        let c = Corners::solved();
        let e = Edges::solved();
        Self {
            c_ori: c.ori_index(),
            e_ori: e.ori_index(),
            z_loc: e.loc_prm(Axis::Z).loc(),
        }
    }

    // pub fn in_subset(&self) -> bool {
    //     self.c_ori == 0 && self.e_ori == 0 && self.z_loc == 494 // TODO: can we make this 0 by changing the definition of slice_loc_index?
    // }

    pub fn index(&self) -> usize {
        let ret = self.c_ori * (Edges::ORI_SIZE * binomial(12, 4))
            + self.e_ori * binomial(12, 4)
            + self.z_loc;
        // assert!(ret < Self::INDEX_SIZE);
        ret
    }

    pub fn from_index(index: usize) -> Self {
        // assert!(index < Self::INDEX_SIZE);
        let mut index = index;
        let z_loc = index % binomial(12, 4);
        index /= binomial(12, 4);
        let e_ori = index % Edges::ORI_SIZE;
        index /= Edges::ORI_SIZE;
        let c_ori = index;
        Self { c_ori, e_ori, z_loc }
    }

    pub fn twisted(&self, twist: Twist) -> Self {
        Self {
            c_ori: TWISTER.twisted_c_ori(self.c_ori, twist),
            e_ori: TWISTER.twisted_e_ori(self.e_ori, twist),
            z_loc: TWISTER.twisted_e_loc_prm(LocPrm::new(self.z_loc, 0), twist).loc(),
        }
    }

    pub fn twisted_by(&self, twists: &[Twist]) -> Self {
        twists
            .iter()
            .fold(*self, |index, &twist| index.twisted(twist))
    }
}

// impl fmt::Display for CosetIndex {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "c_ori: {}, e_ori: {}, z_loc: {}", self.c_ori, self.e_ori, self.z_loc)
//     }
// }

impl Twistable for CosetIndex {
    fn twisted(&self, twist: Twist) -> Self {
        self.twisted(twist)
    }

    fn twisted_by(&self, twists: &[Twist]) -> Self {
        self.twisted_by(twists)
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
