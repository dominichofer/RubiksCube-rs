use super::{TWISTER, SUBSET_INDEX, Twistable, SubsetCube};
use crate::{LocPrm, cubies::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cube {
    c_ori: usize, // 3^7 = 2'187 (defines coset index)
    c_prm: usize, // 8! = 40'320 (defines subset index)
    e_ori: usize, // 2^11 = 2'048 (defines coset index)
    x_loc_prm: LocPrm, // (12 choose 4) * 4! = 11'880 (defines subset index)
    y_loc_prm: LocPrm, // (12 choose 4) * 4! = 11'880 (defines subset index)
    z_loc_prm: LocPrm, // (12 choose 4) * 4! == 11'880 (loc defines coset index, prm defines subset index)
}

impl Cube {
    pub const CORNER_INDEX_SIZE: usize = Corners::ORI_SIZE * Corners::PRM_SIZE; // 88'179'840
    pub const SUBSET_INDEX_SIZE: usize = Corners::PRM_SIZE * factorial(8) * factorial(4) / 2;  // 19'508'428'800
    pub const COSETS_INDEX_SIZE: usize = Corners::ORI_SIZE * Edges::ORI_SIZE * binomial(12, 4); // 2'217'093'120

    pub fn solved() -> Self {
        const C: Corners = Corners::solved();
        const E: Edges = Edges::solved();
        Self {
            c_ori: C.ori_index(),
            c_prm: C.prm_index(),
            e_ori: E.ori_index(),
            x_loc_prm: E.loc_prm(Axis::X),
            y_loc_prm: E.loc_prm(Axis::Y),
            z_loc_prm: E.loc_prm(Axis::Z),
        }
    }

    pub fn corner_index(&self) -> usize {
        self.c_prm * Corners::ORI_SIZE + self.c_ori
    }

    pub fn from_corner_index(index: usize) -> Self {
        assert!(index < Self::CORNER_INDEX_SIZE);
        const E: Edges = Edges::solved();
        Self {
            c_ori: index % Corners::ORI_SIZE,
            c_prm: index / Corners::ORI_SIZE,
            e_ori: E.ori_index(),
            x_loc_prm: E.loc_prm(Axis::X),
            y_loc_prm: E.loc_prm(Axis::Y),
            z_loc_prm: E.loc_prm(Axis::Z),
        }
    }
    
    pub fn subset_cube(&self) -> SubsetCube {
        SubsetCube {
            c_prm: self.c_prm,
            xy_prm: SUBSET_INDEX.e_xy_prm(self.x_loc_prm, self.y_loc_prm),
            z_prm: self.z_loc_prm.prm(),
        }
    }

    pub fn coset_index(&self) -> usize {
        self.c_ori * (Edges::ORI_SIZE * binomial(12, 4))
            + self.e_ori * binomial(12, 4)
            + self.z_loc_prm.loc()
    }

    pub fn from_coset_index(mut index: usize) -> Self {
        assert!(index < Self::COSETS_INDEX_SIZE);
        const C: Corners = Corners::solved();
        const E: Edges = Edges::solved();
        let z_loc = index % binomial(12, 4);
        index /= binomial(12, 4);
        let e_ori = index % Edges::ORI_SIZE;
        index /= Edges::ORI_SIZE;
        let c_ori = index;
        Self {
            c_ori,
            c_prm: C.prm_index(),
            e_ori,
            x_loc_prm: E.loc_prm(Axis::X),
            y_loc_prm: E.loc_prm(Axis::Y),
            z_loc_prm: LocPrm::new(z_loc, E.loc_prm(Axis::Z).prm()),
        }
    }

    pub fn conjugated_by(&self, rot: Axis) -> Self {
        let corners = Corners::from_indices(self.c_prm, self.c_ori).conjugated_by(rot);
        let mut edges = Edges::from_indices(self.x_loc_prm, self.y_loc_prm, self.z_loc_prm, self.e_ori);
        edges = edges.conjugated_by(rot);
        Self {
            c_ori: corners.ori_index(),
            c_prm: corners.prm_index(),
            e_ori: edges.ori_index(),
            x_loc_prm: edges.loc_prm(Axis::X),
            y_loc_prm: edges.loc_prm(Axis::Y),
            z_loc_prm: edges.loc_prm(Axis::Z),
        }
    }

    pub fn inverse(&self) -> Self {
        let corners = Corners::from_indices(self.c_prm, self.c_ori).inverse();
        let mut edges = Edges::from_indices(self.x_loc_prm, self.y_loc_prm, self.z_loc_prm, self.e_ori);
        edges = edges.inverse();
        Self {
            c_ori: corners.ori_index(),
            c_prm: corners.prm_index(),
            e_ori: edges.ori_index(),
            x_loc_prm: edges.loc_prm(Axis::X),
            y_loc_prm: edges.loc_prm(Axis::Y),
            z_loc_prm: edges.loc_prm(Axis::Z),
        }
    }
}

impl Twistable for Cube {
    #[inline(always)]
    fn twisted(&self, twist: Twist) -> Self {
        Self {
            c_ori: TWISTER.twisted_c_ori(self.c_ori, twist),
            c_prm: TWISTER.twisted_c_prm(self.c_prm, twist),
            e_ori: TWISTER.twisted_e_ori(self.e_ori, twist),
            x_loc_prm: TWISTER.twisted_e_loc_prm(self.x_loc_prm, twist),
            y_loc_prm: TWISTER.twisted_e_loc_prm(self.y_loc_prm, twist),
            z_loc_prm: TWISTER.twisted_e_loc_prm(self.z_loc_prm, twist),
        }
    }

    fn twisted_by(&self, twists: &[Twist]) -> Self {
        twists
            .iter()
            .fold(*self, |cube, &twist| cube.twisted(twist))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, RngExt, SeedableRng};

    // Tests 'corner_index' and 'from_corner_index'
    #[test]
    fn test_corners_index() {
        let mut rnd = StdRng::seed_from_u64(42);
        for _ in 0..100_000 {
            let rnd_index = rnd.random_range(0..Cube::CORNER_INDEX_SIZE);
            assert_eq!(Cube::from_corner_index(rnd_index).corner_index(), rnd_index);
        }
    }

    // Tests 'coset_index' and 'from_coset_index'
    #[test]
    fn test_coset_index() {
        let mut rnd = StdRng::seed_from_u64(42);
        for _ in 0..100_000 {
            let rnd_index = rnd.random_range(0..Cube::COSETS_INDEX_SIZE);
            assert_eq!(Cube::from_coset_index(rnd_index).coset_index(), rnd_index);
        }
    }
}