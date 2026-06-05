use super::{TWISTER, Twister, SubsetIndex, CosetIndex};
use crate::{CornerIndex, LocPrm, cubies::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CubeIndex {
    c_ori: usize, // 3^7 = 2'187 (defines coset index)
    c_prm: usize, // 8! = 40'320 (defines subset index)
    e_ori: usize, // 2^11 = 2'048 (defines coset index)
    x_loc_prm: LocPrm, // (12 choose 4) * 4! = 11'880
    y_loc_prm: LocPrm, // (12 choose 4) * 4! = 11'880
    z_loc_prm: LocPrm, // (12 choose 4) * 4! == 11'880
}

impl CubeIndex {
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

    pub fn corner_index(&self) -> usize {
        CornerIndex { prm: self.c_prm, ori: self.c_ori }.index()
    }

    pub fn subset_index(&self) -> SubsetIndex {
        // assert!(self.in_subset(), "Cube is not in the subset: {:?}", self);
        let mut x_loc = [0usize; 4];
        let mut y_loc = [0usize; 4];
        nth_combination2(12, self.x_loc_prm.loc(), &mut x_loc);
        nth_combination2(12, self.y_loc_prm.loc(), &mut y_loc);
        let x_prm = Permutation::<4>::from_index(self.x_loc_prm.prm());
        let y_prm = Permutation::<4>::from_index(self.y_loc_prm.prm());
        let mut prm = [12; 12];
        for i in 0..4 {
            prm[x_loc[i]] = x_prm[i];
            prm[y_loc[i]] = y_prm[i] + 4;
        }
        let mut prm2 = [0; 8];
        let mut j = 0;
        for &p in prm.iter() {
            if p < 8 {
                prm2[j] = p;
                j += 1;
            }
        }
        
        SubsetIndex {
            c_prm: self.c_prm,
            xy_prm: permutation_index(&prm2),
            z_prm: self.z_loc_prm.prm(),
        }
    }

    pub fn coset_index(&self) -> usize {
        CosetIndex { 
            c_ori: self.c_ori,
            e_ori: self.e_ori,
            z_loc: self.z_loc_prm.loc()
         }.index()
    }

    pub fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        let x_loc_prm = twister.twisted_e_loc_prm(self.x_loc_prm, twist);
        let y_loc_prm = twister.twisted_e_loc_prm(self.y_loc_prm, twist);
        let z_loc_prm = twister.twisted_e_loc_prm(self.z_loc_prm, twist);
        CubeIndex {
            c_ori: twister.twisted_c_ori(self.c_ori, twist),
            c_prm: twister.twisted_c_prm(self.c_prm, twist),
            e_ori: twister.twisted_e_ori(self.e_ori, twist),
            x_loc_prm,
            y_loc_prm,
            z_loc_prm,
        }
    }

    pub fn twisted_by(&self, twister: &Twister, twists: &[Twist]) -> Self {
        twists
            .iter()
            .fold(*self, |cube, &twist| cube.twisted(twister, twist))
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
