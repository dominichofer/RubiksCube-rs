use super::{CosetIndex, SubsetIndex, Twister};
use crate::{CornerIndex, cubies::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CubeIndex {
    pub subset: SubsetIndex,
    pub coset: CosetIndex,
}

impl CubeIndex {
    pub fn solved() -> Self {
        Self {
            subset: SubsetIndex::solved(),
            coset: CosetIndex::solved(),
        }
    }

    pub fn corner_index(&self) -> usize {
        CornerIndex {
            prm: self.subset.c_prm,
            ori: self.coset.c_ori,
        }
        .index()
    }

    pub fn in_subset(&self) -> bool {
        self.coset.in_subset()
    }

    pub fn twisted(&self, twister: &Twister, twist: Twist) -> Self {
        CubeIndex {
            subset: SubsetIndex {
                e_slice_prm: twister.twisted_e_slice_prm(self.subset.e_slice_prm, self.coset.e_slice_loc, twist),
                e_non_slice_prm: twister.twisted_e_non_slice_prm(self.subset.e_non_slice_prm, self.coset.e_slice_loc, twist),
                c_prm: twister.twisted_c_prm(self.subset.c_prm, twist),
            },
            coset: self.coset.twisted(twister, twist),
        }
    }

    pub fn twisted_by(&self, twister: &Twister, twists: &[Twist]) -> Self {
        twists
            .iter()
            .fold(*self, |cube, &twist| cube.twisted(twister, twist))
    }

    pub fn conjugated_by(&self, rot: Rotation) -> Self {
        let c_prm = self.subset.c_prm;
        let c_ori = self.coset.c_ori;
        let e_slice_prm = self.subset.e_slice_prm;
        let e_non_slice_prm = self.subset.e_non_slice_prm;
        let e_slice_loc = self.coset.e_slice_loc;
        let e_ori = self.coset.e_ori;

        let corners = Corners::from_indices(c_prm, c_ori).conjugated_by(rot);
        let edges = Edges::from_indices(e_slice_prm, e_non_slice_prm, e_slice_loc, e_ori).conjugated_by(rot);

        Self {
            subset: SubsetIndex {
                c_prm: corners.prm_index(),
                e_slice_prm: edges.slice_prm_index(),
                e_non_slice_prm: edges.non_slice_prm_index(),
            },
            coset: CosetIndex {
                c_ori: corners.ori_index(),
                e_ori: edges.ori_index(),
                e_slice_loc: edges.slice_loc_index(),
            },
        }
    }

    pub fn inverse(&self) -> Self {
        let c_prm = self.subset.c_prm;
        let c_ori = self.coset.c_ori;
        let e_slice_prm = self.subset.e_slice_prm;
        let e_non_slice_prm = self.subset.e_non_slice_prm;
        let e_slice_loc = self.coset.e_slice_loc;
        let e_ori = self.coset.e_ori;

        let corners = Corners::from_indices(c_prm, c_ori).inverse();
        let edges = Edges::from_indices(e_slice_prm, e_non_slice_prm, e_slice_loc, e_ori).inverse();

        Self {
            subset: SubsetIndex {
                c_prm: corners.prm_index(),
                e_slice_prm: edges.slice_prm_index(),
                e_non_slice_prm: edges.non_slice_prm_index(),
            },
            coset: CosetIndex {
                c_ori: corners.ori_index(),
                e_ori: edges.ori_index(),
                e_slice_loc: edges.slice_loc_index(),
            },
        }
    }
}
