use crate::math::is_even_permutation;
use crate::twist::Twist;
use crate::tables::*;
use crate::edges::*;
use crate::corners::*;
use crate::cube::*;
use crate::multi_twister::MultiTwister;

// pub struct CosetCover {
//     subset: DistanceTable,
//     coset_index: usize,
//     max_solution_length: usize,
//     coset: Vec<bool>,
// }

// impl CosetCover {
//     pub fn new(
//         subset: DistanceTable,
//         coset_index: usize,
//         max_solution_length: usize,
//     ) -> Self {
//         Self {
//             subset,
//             coset_index,
//             max_solution_length,
//             coset: [false; SubsetCube::INDEX_SIZE as usize].to_vec(),
//         }
//     }

//     pub fn cover_with(&mut self, twists: &Vec<Twist>) {
//         let mut twister = MultiTwister::new();
//         twister.set_for(twists);

//         for e_non_slice_prm in 0..Edges::NON_SLICE_PRM_SIZE {
//             for e_slice_prm in 0..Edges::SLICE_PRM_SIZE {
//                 let even_prm = is_even_permutation(e_non_slice_prm as i64)
//                     ^ is_even_permutation(e_slice_prm as i64)
//                     ^ true; // in subset e_slice_loc is an even permutation
//                 for c_prm in 0..Corners::PRM_SIZE {
//                     if even_prm != is_even_permutation(c_prm as i64) {
//                         continue;
//                     }
//                     twister.twisted_e_non_slice_prm(e_non_slice_prm, e_slice_loc)
//                 }
//             }
//         }
//     }
// }