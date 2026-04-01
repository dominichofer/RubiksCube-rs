use crate::corners::*;
use crate::edges::*;
use crate::twist::*;
use crate::twister::Twister;
use rayon::prelude::*;

pub struct MultiTwister<'a> {
    twister: &'a Twister,
    c_ori: Vec<u16>,
    c_prm: Vec<u16>,
    e_ori: Vec<u16>,
    e_slice_prm: Vec<u8>,
    e_non_slice_prm: Vec<u16>,
    e_slice_loc: Vec<u16>,
}

impl<'a> MultiTwister<'a> {
    pub fn new(twister: &'a Twister) -> Self {
        Self {
            twister,
            c_ori: vec![0; Corners::ORI_SIZE],
            c_prm: vec![0; Corners::PRM_SIZE],
            e_ori: vec![0; Edges::ORI_SIZE],
            e_slice_prm: vec![0; Edges::SLICE_PRM_SIZE],
            e_non_slice_prm: vec![0; Edges::NON_SLICE_PRM_SIZE],
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
        self.e_slice_prm.par_iter_mut().enumerate().for_each(|(i, entry)| {
            let mut e_slice_prm = i;
            let mut e_slice_loc = Edges::solved().slice_loc_index();
            for twist in twists {
                e_slice_prm = self.twister.twisted_e_slice_prm(e_slice_prm, e_slice_loc, *twist);
                e_slice_loc = self.twister.twisted_e_slice_loc(e_slice_loc, *twist);
            }
            *entry = e_slice_prm as u8;
        });
        self.e_non_slice_prm.par_iter_mut().enumerate().for_each(|(i, entry)| {
            let mut e_non_slice_prm = i;
            let mut e_slice_loc = Edges::solved().slice_loc_index();
            for twist in twists {
                e_non_slice_prm = self.twister.twisted_e_non_slice_prm(e_non_slice_prm, e_slice_loc, *twist);
                e_slice_loc = self.twister.twisted_e_slice_loc(e_slice_loc, *twist);
            }
            *entry = e_non_slice_prm as u16;
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
    pub fn twisted_e_slice_prm(&self, e_slice_prm: usize) -> usize {
        self.e_slice_prm[e_slice_prm] as usize
    }
    pub fn twisted_e_non_slice_prm(&self, e_non_slice_prm: usize) -> usize {
        self.e_non_slice_prm[e_non_slice_prm] as usize
    }
    pub fn twisted_e_slice_loc(&self, e_slice_loc: usize) -> usize {
        self.e_slice_loc[e_slice_loc] as usize
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::twist_set::*;
    use crate::twist_generator::*;

    // Tests 'twisted_c_prm' and 'twisted_c_ori'
    #[test]
    fn test_corners() {
        let twister = Twister::new();
        let mut multi_twister = MultiTwister::new(&twister);
        let mut rnd_full = RandomTwistGen::new(42, TwistSet::full());
        let mut rnd_h0 = RandomTwistGen::new(42, TwistSet::h0());
        for _ in 0..5 {
            let twists = rnd_full.gen_twists(15);
            multi_twister.set_for(&twists);
            
            let mut subset_cube = Corners::solved();
            for _ in 0..1000 {
                let twist_h0 = rnd_h0.gen_twist();
                subset_cube = subset_cube.twisted(twist_h0);
                let subset_prm = subset_cube.prm_index();
                let subset_ori = subset_cube.ori_index();
                let coset_cube = subset_cube.twisted_by(&twists);
                assert_eq!(multi_twister.twisted_c_prm(subset_prm), coset_cube.prm_index());
                assert_eq!(multi_twister.twisted_c_ori(subset_ori), coset_cube.ori_index());
            }
        }
    }

    // Tests 'twisted_e_ori', 'twisted_e_slice_prm', 'twisted_e_non_slice_prm', and 'twisted_e_slice_loc'
    #[test]
    fn test_edges() {
        let twister = Twister::new();
        let mut multi_twister = MultiTwister::new(&twister);
        let mut rnd_full = RandomTwistGen::new(42, TwistSet::full());
        let mut rnd_h0 = RandomTwistGen::new(42, TwistSet::h0());
        for _ in 0..5 {
            let twists = rnd_full.gen_twists(15);
            multi_twister.set_for(&twists);

            let mut subset_cube = Edges::solved();
            for _ in 0..1000 {
                let twist_h0 = rnd_h0.gen_twist();
                subset_cube = subset_cube.twisted(twist_h0);
                let subset_ori = subset_cube.ori_index();
                let subset_slice_prm = subset_cube.slice_prm_index();
                let subset_non_slice_prm = subset_cube.non_slice_prm_index();
                let subset_slice_loc = subset_cube.slice_loc_index();
                assert_eq!(multi_twister.twisted_e_ori(subset_ori), subset_cube.twisted_by(&twists).ori_index());
                assert_eq!(multi_twister.twisted_e_slice_prm(subset_slice_prm), subset_cube.twisted_by(&twists).slice_prm_index());
                assert_eq!(multi_twister.twisted_e_non_slice_prm(subset_non_slice_prm), subset_cube.twisted_by(&twists).non_slice_prm_index());
                assert_eq!(multi_twister.twisted_e_slice_loc(subset_slice_loc), subset_cube.twisted_by(&twists).slice_loc_index());
            }
        }
    }
}
