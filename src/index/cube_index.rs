use super::{TWISTER, SubsetIndex, CosetIndex};
use crate::{CornerIndex, LocPrm, cubies::*};

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
            x_loc_prm: E.x_loc_prm_index(),
            y_loc_prm: E.y_loc_prm_index(),
            z_loc_prm: E.z_loc_prm_index(),
        }
    }

    fn in_subset(&self) -> bool {
        const C: Corners = Corners::solved();
        const E: Edges = Edges::solved();

        self.c_ori == C.ori_index() && self.e_ori == E.ori_index() && self.z_loc_prm.loc() == E.z_loc_prm_index().loc()
    }

    #[inline(always)]
    pub fn corner_index(&self) -> usize {
        CornerIndex { prm: self.c_prm, ori: self.c_ori }.index()
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

    pub fn conjugated_by(&self, rot: Rotation) -> Self {
        let corners = Corners::from_indices(self.c_prm, self.c_ori).conjugated_by(rot);
        let edges = Edges::from_indices(self.x_loc_prm, self.y_loc_prm, self.z_loc_prm, self.e_ori).conjugated_by(rot);

        Self {
            c_ori: corners.ori_index(),
            c_prm: corners.prm_index(),
            e_ori: edges.ori_index(),
            x_loc_prm: edges.x_loc_prm_index(),
            y_loc_prm: edges.y_loc_prm_index(),
            z_loc_prm: edges.z_loc_prm_index(),
        }
    }

    pub fn inverse(&self) -> Self {
        let corners = Corners::from_indices(self.c_prm, self.c_ori).inverse();
        let edges = Edges::from_indices(self.x_loc_prm, self.y_loc_prm, self.z_loc_prm, self.e_ori).inverse();
        Self {
            c_ori: corners.ori_index(),
            c_prm: corners.prm_index(),
            e_ori: edges.ori_index(),
            x_loc_prm: edges.x_loc_prm_index(),
            y_loc_prm: edges.y_loc_prm_index(),
            z_loc_prm: edges.z_loc_prm_index(),
        }
    }
}
