use crate::corners::*;
use crate::edges::*;
use crate::twist::*;
use rayon::prelude::*;

pub trait Twistable: Clone {
    fn twisted(&self, twister: &Twister, twist: Twist) -> Self;

    fn twisted_by(&self, twister: &Twister, twists: &[Twist]) -> Self {
        twists.iter().fold(self.clone(), |cube, &twist| {
            cube.twisted(twister, twist)
        })   
    }
}

// pub struct SliceLocPermutationParity {
//     table: Vec<bool>,
// }

// impl SliceLocPermutationParity {
//     pub fn new() -> Self {
//         let mut table = vec![false; Edges::SLICE_LOC_SIZE as usize];
//         let e_solved = Edges::solved();
//         for i in 0..Edges::SLICE_LOC_SIZE {
//             let e = Edges::from_index(
//                 e_solved.slice_prm_index(),
//                 e_solved.non_slice_prm_index(),
//                 i as u16,
//                 e_solved.ori_index(),
//             );
//             table[i as usize] = is_even_permutation_array(&e.cubies());
//         }
//         Self { table }
//     }

//     pub fn get(&self, index: u16) -> bool {
//         self.table[index as usize]
//     }
// }

#[derive(Clone)]
pub struct Twister {
    c_ori: Vec<u16>,
    c_prm: Vec<u16>,
    e_ori: Vec<u16>,
    e_slice_prm: Vec<u8>,
    e_non_slice_prm: Vec<u16>,
    e_slice_loc: Vec<u16>,
}

const COUNT: usize = TwistSet::full_and_none().count();

impl Twister {
    pub fn new() -> Self {
        let mut c_ori = vec![0u16; Corners::ORI_SIZE * COUNT];
        let mut c_prm = vec![0u16; Corners::PRM_SIZE * COUNT];
        let mut e_ori = vec![0u16; Edges::ORI_SIZE * COUNT];
        let mut e_slice_prm = vec![0u8; Edges::SLICE_PRM_SIZE * Edges::SLICE_LOC_SIZE * COUNT];
        let mut e_non_slice_prm = vec![0u16; Edges::NON_SLICE_PRM_SIZE * Edges::SLICE_LOC_SIZE * COUNT];
        let mut e_slice_loc = vec![0u16; Edges::SLICE_LOC_SIZE * COUNT];

        c_ori
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Corners::from_indices(0, i);
                for twist in TwistSet::full_and_none().iter() {
                    chunk[twist as usize] = obj.twisted(twist).ori_index() as u16;
                }
            });
        c_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Corners::from_indices(i, 0);
                for twist in TwistSet::full_and_none().iter() {
                    chunk[twist as usize] = obj.twisted(twist).prm_index() as u16;
                }
            });
        e_ori
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_indices(0, 0, 0, i);
                for twist in TwistSet::full_and_none().iter() {
                    chunk[twist as usize] = obj.twisted(twist).ori_index() as u16;
                }
            });
        e_slice_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_indices(i / Edges::SLICE_LOC_SIZE, 0, i % Edges::SLICE_LOC_SIZE, 0);
                for twist in TwistSet::full_and_none().iter() {
                    chunk[twist as usize] = obj.twisted(twist).slice_prm_index() as u8;
                }
            });
        e_non_slice_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_indices(0, i / Edges::SLICE_LOC_SIZE, i % Edges::SLICE_LOC_SIZE, 0);
                for twist in TwistSet::full_and_none().iter() {
                    chunk[twist as usize] = obj.twisted(twist).non_slice_prm_index() as u16;
                }
            });
        e_slice_loc
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_indices(0, 0, i, 0);
                for twist in TwistSet::full_and_none().iter() {
                    chunk[twist as usize] = obj.twisted(twist).slice_loc_index() as u16;
                }
            });

        Self {
            c_ori,
            c_prm,
            e_ori,
            e_slice_prm,
            e_non_slice_prm,
            e_slice_loc,
        }
    }

    pub fn twisted_c_ori(&self, c_ori: usize, twist: Twist) -> usize {
        self.c_ori[c_ori * COUNT + twist as usize] as usize
    }
    pub fn twisted_c_prm(&self, c_prm: usize, twist: Twist) -> usize {
        self.c_prm[c_prm * COUNT + twist as usize] as usize
    }
    pub fn twisted_e_ori(&self, e_ori: usize, twist: Twist) -> usize {
        self.e_ori[e_ori * COUNT + twist as usize] as usize
    }
    pub fn twisted_e_slice_prm(&self, e_slice_prm: usize, e_slice_loc: usize, twist: Twist) -> usize {
        self.e_slice_prm[(e_slice_prm * Edges::SLICE_LOC_SIZE + e_slice_loc) * COUNT + twist as usize] as usize
    }
    pub fn twisted_e_non_slice_prm(&self, e_non_slice_prm: usize, e_slice_loc: usize, twist: Twist) -> usize {
        self.e_non_slice_prm[(e_non_slice_prm * Edges::SLICE_LOC_SIZE + e_slice_loc) * COUNT + twist as usize] as usize
    }
    pub fn twisted_e_slice_loc(&self, e_slice_loc: usize, twist: Twist) -> usize {
        self.e_slice_loc[e_slice_loc * COUNT + twist as usize] as usize
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Tests 'twisted_c_prm' and 'twisted_c_ori'
    #[test]
    fn test_corners() {
        let twister = Twister::new();
        let mut rnd = RandomTwistGen::new(42, TwistSet::full());
        let mut c = Corners::solved();
        let mut prm = c.prm_index();
        let mut ori = c.ori_index();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            c = c.twisted(twist);
            prm = twister.twisted_c_prm(prm, twist);
            ori = twister.twisted_c_ori(ori, twist);
            assert_eq!(c.prm_index(), prm);
            assert_eq!(c.ori_index(), ori);
        }
    }

    // Tests 'twisted_e_ori', 'twisted_e_slice_prm', 'twisted_e_non_slice_prm', and 'twisted_e_slice_loc'
    #[test]
    fn test_edges() {
        let twister = Twister::new();
        let mut rnd = RandomTwistGen::new(42, TwistSet::full());
        let mut e = Edges::solved();
        let mut ori = e.ori_index();
        let mut slice_prm = e.slice_prm_index();
        let mut non_slice_prm = e.non_slice_prm_index();
        let mut slice_loc = e.slice_loc_index();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            e = e.twisted(twist);
            ori = twister.twisted_e_ori(ori, twist);
            slice_prm = twister.twisted_e_slice_prm(slice_prm, slice_loc, twist);
            non_slice_prm = twister.twisted_e_non_slice_prm(non_slice_prm, slice_loc, twist);
            slice_loc = twister.twisted_e_slice_loc(slice_loc, twist);
            assert_eq!(e.ori_index(), ori);
            assert_eq!(e.slice_prm_index(), slice_prm);
            assert_eq!(e.non_slice_prm_index(), non_slice_prm);
            assert_eq!(e.slice_loc_index(), slice_loc);
        }
    }
}
