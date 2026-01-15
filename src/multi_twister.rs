use crate::corners::*;
use crate::edges::*;
use crate::twist::*;
use crate::twister::Twister;
use crate::cube::*;
use rayon::prelude::*;

pub struct MultiTwister {
    twister: Twister,
    c_ori: Vec<u16>,
    c_prm: Vec<u16>,
    e_ori: Vec<u16>,
    e_slice_prm: Vec<u8>,
    e_non_slice_prm: Vec<u16>,
    e_slice_loc: Vec<u16>,
}

impl MultiTwister {
    pub fn new(twister: Twister) -> Self {
        Self {
            twister,
            c_ori: vec![0; Corners::ORI_SIZE],
            c_prm: vec![0; Corners::PRM_SIZE],
            e_ori: vec![0; Edges::ORI_SIZE],
            e_slice_prm: vec![0; Edges::SLICE_PRM_SIZE * Edges::SLICE_LOC_SIZE],
            e_non_slice_prm: vec![0; Edges::NON_SLICE_PRM_SIZE * Edges::SLICE_LOC_SIZE],
            e_slice_loc: vec![0; Edges::SLICE_LOC_SIZE],
        }
    }

    pub fn set_for(&mut self, twists: &[Twist]) {
        self.c_ori.par_iter_mut().enumerate().for_each(|(i, entry)| {
            let mut index = i;
            for twist in twists {
                index = self.twister.twisted_c_ori(index, *twist);
            }
            *entry = index as u16;
        });
        self.c_prm.par_iter_mut().enumerate().for_each(|(i, entry)| {
            let mut index = i;
            for twist in twists {
                index = self.twister.twisted_c_prm(index, *twist);
            }
            *entry = index as u16;
        });
        self.e_ori.par_iter_mut().enumerate().for_each(|(i, entry)| {
            let mut index = i;
            for twist in twists {
                index = self.twister.twisted_e_ori(index, *twist);
            }
            *entry = index as u16;
        });
        self.e_slice_prm.par_chunks_mut(Edges::SLICE_LOC_SIZE as usize).enumerate().for_each(|(i, chunk)| {
            chunk.par_iter_mut().enumerate().for_each(|(j, entry)| {
            let mut e_slice_prm = i;
            let mut e_slice_loc = j;
            for twist in twists {
                e_slice_prm = self.twister.twisted_e_slice_prm(e_slice_prm, e_slice_loc, *twist);
                e_slice_loc = self.twister.twisted_e_slice_loc(e_slice_loc, *twist);
            }
            *entry = e_slice_prm as u8;
            });
        });
        self.e_non_slice_prm.par_chunks_mut(Edges::SLICE_LOC_SIZE as usize).enumerate().for_each(|(i, chunk)| {
            chunk.par_iter_mut().enumerate().for_each(|(j, entry)| {
            let mut e_non_slice_prm = i;
            let mut e_slice_loc = j;
            for twist in twists {
                e_non_slice_prm = self.twister.twisted_e_non_slice_prm(e_non_slice_prm, e_slice_loc, *twist);
                e_slice_loc = self.twister.twisted_e_slice_loc(e_slice_loc, *twist);
            }
            *entry = e_non_slice_prm as u16;
            });
        });
        self.e_slice_loc.par_iter_mut().enumerate().for_each(|(i, entry)| {
            let mut index = i;
            for twist in twists {
                index = self.twister.twisted_e_slice_loc(index, *twist);
            }
            *entry = index as u16;
        });
    }

    pub fn twisted_c_ori(&self, c_ori: usize) -> usize {
        self.c_ori[c_ori] as usize
    }
    pub fn twisted_c_prm(&self, c_prm: usize) -> usize {
        self.c_prm[c_prm] as usize
    }
    pub fn twisted_e_ori(&self, e_ori: usize) -> usize {
        self.e_ori[e_ori] as usize
    }
    pub fn twisted_e_slice_prm(&self, e_slice_prm: usize, e_slice_loc: usize) -> usize {
        self.e_slice_prm[e_slice_prm * Edges::SLICE_LOC_SIZE as usize + e_slice_loc] as usize
    }
    pub fn twisted_e_non_slice_prm(&self, e_non_slice_prm: usize, e_slice_loc: usize) -> usize {
        self.e_non_slice_prm[e_non_slice_prm * Edges::SLICE_LOC_SIZE as usize + e_slice_loc] as usize
    }
    pub fn twisted_e_slice_loc(&self, e_slice_loc: usize) -> usize {
        self.e_slice_loc[e_slice_loc] as usize
    }
    pub fn twisted_corners_cube(&self, cube: &CornersCube) -> CornersCube {
        CornersCube {
            prm: self.twisted_c_prm(cube.prm) as usize,
            ori: self.twisted_c_ori(cube.ori) as usize,
        }
    }
    pub fn twisted_subset_cube(&self, cube: &SubsetCube) -> SubsetCube {
        SubsetCube {
            c_prm: self.twisted_c_prm(cube.c_prm) as usize,
            e_slice_prm: self.twisted_e_slice_prm(cube.e_slice_prm, 0) as usize,
            e_non_slice_prm: self.twisted_e_non_slice_prm(cube.e_non_slice_prm, 0) as usize,
        }
    }
    pub fn twisted_coset_cube(&self, cube: &CosetCube) -> CosetCube {
        CosetCube {
            c_ori: self.twisted_c_ori(cube.c_ori) as usize,
            e_ori: self.twisted_e_ori(cube.e_ori) as usize,
            e_slice_loc: self.twisted_e_slice_loc(cube.e_slice_loc) as usize,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Tests 'twisted_c_prm' and 'twisted_c_ori'
    #[test]
    fn test_corners() {
        let twister = Twister::new();
        let mut multi_twister = MultiTwister::new(twister);
        let mut rnd = RandomTwistGen::new(42, TwistSet::full());
        let mut c = Corners::solved();
        let mut prm = c.prm_index();
        let mut ori = c.ori_index();
        for _ in 0..5 {
            let twists = rnd.gen_twists(5);
            multi_twister.set_for(&twists);
            for _ in 0..1000 {
                c = c.twisted_by(&twists);
                prm = multi_twister.twisted_c_prm(prm);
                ori = multi_twister.twisted_c_ori(ori);
                assert_eq!(c.prm_index(), prm);
                assert_eq!(c.ori_index(), ori);
            }
        }
    }

    // Tests 'twisted_e_ori', 'twisted_e_slice_prm', 'twisted_e_non_slice_prm', and 'twisted_e_slice_loc'
    #[test]
    fn test_edges() {
        let mut multi_twister = MultiTwister::new(Twister::new());
        let mut rnd = RandomTwistGen::new(43, TwistSet::full());
        let mut e = Edges::solved();
        let mut ori = e.ori_index();
        let mut slice_prm = e.slice_prm_index();
        let mut non_slice_prm = e.non_slice_prm_index();
        let mut slice_loc = e.slice_loc_index();
        for _ in 0..5 {
            let twists = rnd.gen_twists(5);
            multi_twister.set_for(&twists);
            for _ in 0..1000 {
                e = e.twisted_by(&twists);
                ori = multi_twister.twisted_e_ori(ori);
                slice_prm = multi_twister.twisted_e_slice_prm(slice_prm, slice_loc);
                non_slice_prm = multi_twister.twisted_e_non_slice_prm(non_slice_prm, slice_loc);
                slice_loc = multi_twister.twisted_e_slice_loc(slice_loc);
                assert_eq!(e.ori_index(), ori);
                assert_eq!(e.slice_prm_index(), slice_prm);
                assert_eq!(e.non_slice_prm_index(), non_slice_prm);
                assert_eq!(e.slice_loc_index(), slice_loc);
            }
        }
    }
}
