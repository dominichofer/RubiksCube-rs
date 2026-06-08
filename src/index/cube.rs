use super::{TWISTER, SubsetIndex, CosetIndex};
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

    pub fn from_corner_index(index: usize) -> Self {
        assert!(index < Corners::PRM_SIZE * Corners::ORI_SIZE);
        const E: Edges = Edges::solved();
        Self {
            c_prm: index / Corners::ORI_SIZE,
            c_ori: index % Corners::ORI_SIZE,
            e_ori: E.ori_index(),
            x_loc_prm: E.loc_prm(Axis::X),
            y_loc_prm: E.loc_prm(Axis::Y),
            z_loc_prm: E.loc_prm(Axis::Z),
        }
    }

    #[inline(always)]
    pub fn corner_index(&self) -> usize {
        self.c_prm * Corners::ORI_SIZE + self.c_ori
    }

    #[inline(always)]
    pub fn subset_index(&self) -> SubsetIndex {
        SubsetIndex {
            c_prm: self.c_prm,
            xy_prm: TWISTER.e_xy_prm(self.x_loc_prm, self.y_loc_prm),
            z_prm: self.z_loc_prm.prm(),
        }
    }

    #[inline(always)]
    pub fn coset_index(&self) -> usize {
        CosetIndex { 
            c_ori: self.c_ori,
            e_ori: self.e_ori,
            z_loc: self.z_loc_prm.loc()
         }.index()
    }

    #[inline(always)]
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
