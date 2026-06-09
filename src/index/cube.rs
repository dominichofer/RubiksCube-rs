use super::{TWISTER, Twistable, SubsetIndex};
use crate::{LocPrm, cubies::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cube {
    c_ori: usize, // 3^7 = 2'187 (defines coset index)
    c_prm: usize, // 8! = 40'320 (defines subset index)
    e_ori: usize, // 2^11 = 2'048 (defines coset index)
    x_loc_prm: LocPrm, // (12 choose 4) * 4! = 11'880
    y_loc_prm: LocPrm, // (12 choose 4) * 4! = 11'880
    z_loc_prm: LocPrm, // (12 choose 4) * 4! == 11'880
}

impl Cube {
    pub const CORNER_INDEX_SIZE: usize = Corners::ORI_SIZE * Corners::PRM_SIZE; // 88'179'840
    pub const SUBSET_INDEX_SIZE: usize = Corners::PRM_SIZE / 2 * factorial(8) * factorial(4);  // 19'508'428'800
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

    pub fn subset_index(&self) -> SubsetIndex {
        SubsetIndex {
            c_prm: self.c_prm,
            xy_prm: TWISTER.e_xy_prm(self.x_loc_prm, self.y_loc_prm),
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

    pub fn twisted(&self, twist: Twist) -> Self {
        Cube {
            c_ori: TWISTER.twisted_c_ori(self.c_ori, twist),
            c_prm: TWISTER.twisted_c_prm(self.c_prm, twist),
            e_ori: TWISTER.twisted_e_ori(self.e_ori, twist),
            x_loc_prm: TWISTER.twisted_e_loc_prm(self.x_loc_prm, twist),
            y_loc_prm: TWISTER.twisted_e_loc_prm(self.y_loc_prm, twist),
            z_loc_prm: TWISTER.twisted_e_loc_prm(self.z_loc_prm, twist),
        }
    }

    pub fn twisted_by(&self, twists: &[Twist]) -> Self {
        twists
            .iter()
            .fold(*self, |cube, &twist| cube.twisted(twist))
    }

    pub fn conjugated_by(&self, rot: Axis) -> Self {
        let corners = Corners::from_indices(self.c_prm, self.c_ori).conjugated_by(rot);
        let edges = Edges::from_indices(self.x_loc_prm, self.y_loc_prm, self.z_loc_prm, self.e_ori).conjugated_by(rot);

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
        let edges = Edges::from_indices(self.x_loc_prm, self.y_loc_prm, self.z_loc_prm, self.e_ori).inverse();
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

    // Tests 'corner_index' and 'from_corner_index'
    #[test]
    fn test_corners_index() {
        let mut rnd = StdRng::seed_from_u64(42);
        for _ in 0..100_000 {
            let rnd_index = rnd.random_range(0..Cube::CORNER_INDEX_SIZE);
            assert_eq!(Cube::from_corner_index(rnd_index).corner_index(), rnd_index);
        }
    }

    // Tests 'subset_index' and 'from_subset_index'
    // #[test]
    // fn test_subset_index() {
    //     let mut rnd = StdRng::seed_from_u64(42);
    //     for _ in 0..100_000 {
    //         let rnd_index = rnd.random_range(0..Cube::SUBSET_INDEX_SIZE);
    //         assert_eq!(Cube::from_subset_index(rnd_index).subset_index(), rnd_index);
    //     }
    // }

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