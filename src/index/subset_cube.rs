use super::Twistable;
use crate::TWISTER;
use crate::SUBSET_TWISTER;
use crate::cubies::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubsetCube {
    pub c_prm: usize, // 8! = 40'320
    pub xy_prm: usize, // 8! = 40'320
    pub z_prm: usize, // 4! = 24
}

impl SubsetCube {
    pub const INDEX_SIZE: usize = Corners::PRM_SIZE / 2 * factorial(8) * factorial(4);  // 19'508'428'800

    pub fn solved() -> Self {
        let c = Corners::solved();
        let e = Edges::solved();
        Self {
            c_prm: c.prm_index(),
            xy_prm: e.xy_prm_index(),
            z_prm: e.loc_prm(Axis::Z).prm(),
        }
    }

    pub fn index(&self) -> usize {
        (self.c_prm / 2) * factorial(8) * factorial(4)
            + self.xy_prm * factorial(4)
            + self.z_prm
    }

    pub fn from_index(mut index: usize) -> Self {
        assert!(index < Self::INDEX_SIZE);
        let z_prm = index % factorial(4); index /= factorial(4);
        let xy_prm = index % factorial(8); index /= factorial(8);
        let mut c_prm = index * 2;
        let e_even_prm = is_even_permutation(xy_prm)
            ^ is_even_permutation(z_prm)
            ^ true; // in subset z_prm is an even permutation
        if e_even_prm != is_even_permutation(c_prm) {
            c_prm += 1;
        }
        Self { c_prm, xy_prm, z_prm }
    }
}

impl Twistable for SubsetCube {
    fn twisted(&self, twist: Twist) -> Self {
        Self {
            c_prm: TWISTER.twisted_c_prm(self.c_prm, twist),
            xy_prm: SUBSET_TWISTER.twisted_xy_prm(self.xy_prm, twist),
            z_prm: SUBSET_TWISTER.twisted_z_prm(self.z_prm, twist),
        }
    }
    fn twisted_by(&self, twists: &[Twist]) -> Self {
        twists
            .iter()
            .fold(*self, |index, &twist| index.twisted(twist))
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
            let rnd_index = rnd.random_range(0..SubsetCube::INDEX_SIZE);
            assert_eq!(SubsetCube::from_index(rnd_index).index(), rnd_index);
        }
    }
}
